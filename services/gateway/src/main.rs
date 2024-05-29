use std::future::{IntoFuture, poll_fn};
use std::net::SocketAddr;
use std::task::Poll;

use axum::extract::{ConnectInfo, Query, WebSocketUpgrade};
use axum::extract::ws::WebSocket;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use clap::Parser;
use error_stack::{Report, ResultExt};
use futures::{FutureExt, SinkExt, StreamExt};
use log::{error, info};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

use common::{error_object, with_bootstrap, };
use database::init_database;
use crate::packets::{WsPacketClientbound, WsPacketServerbound};
use crate::session::{RunSessionError, send_message, SocketSession};
use crate::startup::StartupParams;

mod packets;
mod session;
mod startup;

error_object!(ApplicationError, "Failed to start gateway");

with_bootstrap!(run_server, StartupParams);

error_object!(ServerError, "Failed to run server");

async fn run_server(params: StartupParams) -> error_stack::Result<(), ServerError> {
    let database = init_database(&params.database_params)
        .await
        .change_context(ServerError)?;

    let router = Router::new().route("/", get(handle_websocket));

    if params.binds.is_empty() {
        error!("No binds specified");
        return Err(Report::new(ServerError)
            .attach_printable("No addresses specified to bind the web server to"));
    }

    let mut servers = Vec::with_capacity(params.binds.len());
    for bind in params.binds.iter() {
        let router = router.clone();
        let listener = TcpListener::bind(bind).await.change_context(ServerError)?;
        info!("Listening on http://{}", listener.local_addr().unwrap());
        let server = axum::serve(
            listener,
            router.into_make_service_with_connect_info::<SocketAddr>(),
        );
        servers.push(server.into_future());
    }
    poll_fn(|cx| {
        for server in servers.iter_mut() {
            let poll = server.poll_unpin(cx);
            if let Poll::Ready(outcome) = poll {
                return Poll::Ready(outcome);
            }
        }
        Poll::Pending
    })
        .await
        .change_context(ServerError)?;

    Ok(())
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct WebSocketQueryParams {
    #[serde(default, alias = "dev")]
    pub debug: bool,
    #[serde(alias = "e", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<Encoding>,
    //since we are at the first version, we don't need to specify the version yet
    //pub version: Option<u32>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
enum Encoding {
    #[default]
    #[serde(alias = "javascript_object_notation")]
    Json,
    #[serde(alias = "msp", alias = "mp", alias = "msgpack")]
    MessagePack,
    #[serde(alias = "rust_object_notation")]
    Ron,
    #[serde(alias = "extensible_markup_language")]
    Xml,
    #[serde(alias = "concise-binary-object-representation")]
    Cbor,
}

async fn handle_websocket(
    ws: WebSocketUpgrade,
    params: Query<WebSocketQueryParams>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let params = params.0;
    ws.protocols(["kiwi"])
        .on_upgrade(move |socket| handle_socket(socket, params))
}

async fn handle_socket(mut socket: WebSocket, params: WebSocketQueryParams) {
    let run_socket_result = run_websocket(&mut socket, &params).await;
    if let Err(report) = run_socket_result {
        let encoding = params.encoding.unwrap_or_default();
        let _ = send_message(
            encoding,
            &mut socket,
            &WsPacketServerbound::CriticalError(report),
        )
            .await;
    }
    let _ = socket.close().await;
}

error_object!(SocketError, "Failed to handle socket");
async fn run_websocket(
    socket: &mut WebSocket,
    params: &WebSocketQueryParams,
) -> error_stack::Result<(), SocketError> {
    SocketSession::run(socket, params, |mut session| async move {
        loop {
            let message = session.receive().await.change_context(RunSessionError)?;
            match message {
                WsPacketClientbound::Echo(message) => {
                    session
                        .send(&WsPacketServerbound::Hello(message))
                        .await
                        .change_context(RunSessionError)?;
                }
                WsPacketClientbound::SubscribeMessages(subscribe) => {
                    
                }
            }
        }
    })
        .await
        .change_context(SocketError)?;

    Ok(())
}
