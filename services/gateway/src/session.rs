use std::future::{Future, poll_fn};
use std::task::Poll;

use axum::extract::ws::{Message, WebSocket};
use error_stack::{Report, ResultExt};
use futures::{FutureExt, SinkExt, StreamExt};
use tokio::task::JoinError;

use common::error_object;

use crate::Encoding;
use crate::packets::{WsPacketClientbound, WsPacketServerbound};

#[derive(Debug, Clone)]
pub enum DataMessage {
    Text(String),
    Binary(Vec<u8>),
}

impl From<DataMessage> for Message {
    fn from(value: DataMessage) -> Self {
        match value {
            DataMessage::Text(text) => Message::Text(text),
            DataMessage::Binary(binary) => Message::Binary(binary),
        }
    }
}

#[derive()]
pub struct SocketSession {
    encoding: Encoding,
    pub send: tokio::sync::mpsc::Sender<Message>,
    pub receive: tokio::sync::broadcast::Receiver<DataMessage>,
}

impl SocketSession {
    pub async fn send(&mut self, message: &WsPacketServerbound) -> error_stack::Result<(), SendMessageError> {
        let message = create_message(self.encoding, message)
            .change_context(SendMessageError)?;
        self.send.send(message.into()).await
            .change_context(SendMessageError)?;
        Ok(())
    }
    pub async fn receive(&mut self) -> error_stack::Result<WsPacketClientbound, ReadMessageError> {
        let message = self.receive.recv().await
            .change_context(ReadMessageError::DeserializeError)?;
        let decoded = read_message(self.encoding, message).await
            .change_context(ReadMessageError::DeserializeError)?;
        Ok(decoded)
    }
}

impl Clone for SocketSession {
    fn clone(&self) -> Self {
        let cloned_send = self.send.clone();
        let cloned_receive = self.receive.resubscribe();
        Self {
            encoding: self.encoding,
            send: cloned_send,
            receive: cloned_receive,
        }
    }
}

error_object!(StartSessionError, "Failed to start session");
error_object!(RunSessionError, "Failed to run session");
impl SocketSession {
    pub async fn run<F, Fut>(socket: &mut WebSocket, encoding: Encoding, use_session: F) -> error_stack::Result<(), StartSessionError>
        where F: FnOnce(SocketSession) -> Fut + Send + 'static,
              Fut: Future<Output=error_stack::Result<(), RunSessionError>> + Send + 'static,
    {
        let (send_channel, mut send_pipe) = tokio::sync::mpsc::channel(128);
        let (receive_pipe, receive_channel) = tokio::sync::broadcast::channel(128);
        let session = SocketSession {
            encoding,
            send: send_channel,
            receive: receive_channel,
        };
        let mut task = tokio::spawn(use_session(session));

        while let Some(received) = socket_loop(socket, &mut send_pipe, &mut task).await {
            match received {
                LoopResult::Send(send) =>
                    socket.send(send.into()).await
                        .change_context(StartSessionError)?,

                LoopResult::Receive(received) => {
                    match received {
                        Message::Text(text) => {
                            receive_pipe.send(DataMessage::Text(text))
                                .change_context(StartSessionError)?;
                        }
                        Message::Binary(binary) => {
                            receive_pipe.send(DataMessage::Binary(binary))
                                .change_context(StartSessionError)?;
                        }
                        Message::Ping(ping) => {
                            socket.send(Message::Pong(ping)).await
                                .change_context(StartSessionError)?;
                        }
                        Message::Pong(pong) => {}
                        Message::Close(close) => break
                    }
                }
                LoopResult::Close => break,
                LoopResult::TaskComplete(outcome) => {
                    outcome.change_context(StartSessionError)?
                        .change_context(StartSessionError)?;
                    break;
                }
            }
        }
        send_pipe.close();
        Ok(())
    }
}

enum LoopResult {
    Send(Message),
    Receive(Message),
    TaskComplete(Result<error_stack::Result<(), RunSessionError>, JoinError>),
    Close,
}

async fn socket_loop(
    socket: &mut WebSocket,
    send_pipe: &mut tokio::sync::mpsc::Receiver<Message>,
    task: &mut tokio::task::JoinHandle<error_stack::Result<(), RunSessionError>>,
) -> Option<LoopResult> {
    //let mut receive = Box::pin(socket.next());
    //let mut send = Box::pin(send_pipe.recv());
    let mut socket = Box::pin(socket);
    poll_fn(move |cx| {
        let poll = task.poll_unpin(cx);
        if let Poll::Ready(result) = poll {
            //TODO: handle error
            return Poll::Ready(Some(LoopResult::TaskComplete(result)));
        }


        let socket_send_poll = send_pipe.poll_recv(cx);
        if let Poll::Ready(to_send) = socket_send_poll {
            return if let Some(message) = to_send {
                Poll::Ready(Some(LoopResult::Send(message)))
            } else {
                Poll::Ready(Some(LoopResult::Close))
            };
        }

        let socket_receive_poll = socket.poll_next_unpin(cx);
        if let Poll::Ready(received) = socket_receive_poll {
            return if let Some(Ok(message)) = received {
                Poll::Ready(Some(LoopResult::Receive(message)))
            } else {
                Poll::Ready(Some(LoopResult::Close))
            };
        }

        Poll::Pending
    }).await
}



error_object!(SendMessageError, "Failed to send message");

pub async fn send_message<P, T, E>(encoding: Encoding, sink: &mut P, message: &T) -> error_stack::Result<(), SendMessageError>
    where P: futures::Sink<Message, Error=E> + Unpin,
          T: serde::Serialize + ?Sized,
          E: std::error::Error + Send + Sync + 'static
{
    let message = create_message(encoding, message)
        .change_context(SendMessageError)?;
    sink.send(message.into()).await
        .change_context(SendMessageError)?;
    Ok(())
}

error_object!(CreateMessageError, "Failed to create message");
pub fn create_message<T>(encoding: Encoding, message: &T) -> error_stack::Result<DataMessage, CreateMessageError>
    where T: serde::Serialize + ?Sized
{
    let message = match encoding {
        Encoding::Json => DataMessage::Text(serde_json::to_string(message)
            .change_context(CreateMessageError)?),
        Encoding::MessagePack => DataMessage::Binary(rmp_serde::to_vec_named(message)
            .change_context(CreateMessageError)?),
        Encoding::MessagePacketPositional => DataMessage::Binary(rmp_serde::to_vec(message)
            .change_context(CreateMessageError)?),
        Encoding::Ron => DataMessage::Text(ron::ser::to_string(message)
            .change_context(CreateMessageError)?),
        Encoding::Xml => DataMessage::Text(quick_xml::se::to_string(message)
            .change_context(CreateMessageError)?),
    };
    Ok(message)
}

#[derive(Debug)]
pub enum ReadMessageError {
    ExpectedText,
    ExpectedBinary,
    DeserializeError,
}

impl std::fmt::Display for ReadMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadMessageError::ExpectedText => write!(f, "Expected text message byt got binary"),
            ReadMessageError::ExpectedBinary => write!(f, "Expected binary message byt got text"),
            ReadMessageError::DeserializeError => write!(f, "Failed to deserialize message"),
        }
    }
}

impl std::error::Error for ReadMessageError {}

async fn read_message<T>(encoding: Encoding, message: DataMessage) -> error_stack::Result<T, ReadMessageError>
    where T: serde::de::DeserializeOwned {
    let decoded = match message {
        DataMessage::Text(text) => {
            match encoding {
                Encoding::Json => serde_json::from_str::<T>(&text)
                    .change_context(ReadMessageError::DeserializeError)?,
                Encoding::Ron => ron::de::from_str::<T>(&text)
                    .change_context(ReadMessageError::DeserializeError)?,
                Encoding::Xml => quick_xml::de::from_str::<T>(&text)
                    .change_context(ReadMessageError::DeserializeError)?,
                _ => return Err(Report::new(ReadMessageError::ExpectedText)),
            }
        }
        DataMessage::Binary(binary) => {
            match encoding {
                Encoding::MessagePack | Encoding::MessagePacketPositional => rmp_serde::from_slice(&binary)
                    .change_context(ReadMessageError::DeserializeError)?,
                _ => return Err(Report::new(ReadMessageError::ExpectedBinary)),
            }
        }
    };
    Ok(decoded)
}