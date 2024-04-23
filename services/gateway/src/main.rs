use std::net::SocketAddr;

use axum::extract::{ConnectInfo, Query, WebSocketUpgrade};
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use clap::Parser;
use error_stack::{FutureExt, ResultExt};
use futures::{SinkExt, StreamExt};
use log::{error, info};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::time::sleep;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use common::{error_object, init_logger};

use crate::packets::{WsPacketClientbound, WsPacketServerbound};
use crate::session::{RunSessionError, send_message, SocketSession};
use crate::startup::StartupParams;

mod startup;
mod packets;
mod session;

error_object!(ApplicationError, "Failed to start gateway");

#[tokio::main]
async fn main() -> error_stack::Result<(), ApplicationError> {
    let params = StartupParams::parse();
    init_logger(params.log_level)
        .change_context(ApplicationError)?;

    let run_server_result = run_server(params).await;
    fast_log::flush()
        .change_context(ApplicationError)?;
    fast_log::exit()
        .change_context(ApplicationError)?;
    run_server_result
        .change_context(ApplicationError)?;
    Ok(())
}

error_object!(ServerError, "Failed to run server");

async fn run_server(
    params: StartupParams,
) -> error_stack::Result<(), ServerError> {
    let router = Router::new()
        .route("/", get(handle_websocket));

    let listener = TcpListener::bind(params.bind)
        .await
        .change_context(ServerError)?;
    info!("local addr =  {}", listener.local_addr().unwrap());
    axum::serve(listener, router.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .change_context(ServerError)?;

    Ok(())
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct WebSocketParams {
    #[serde(alias = "e")]
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
    #[serde(alias = "msp", alias = "msgpack")]
    MessagePack,
    //like messagepacket, but instead of using named fields, 
    // it uses positional fields making it more compact but harder to implement
    #[serde(alias = "mp", alias = "msgpack_p")]
    MessagePacketPositional,
    #[serde(alias = "rust_object_notation")]
    Ron,
    #[serde(alias = "extensible_markup_language")]
    Xml,
}

async fn handle_websocket(
    ws: WebSocketUpgrade,
    params: Query<WebSocketParams>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let params = params.0;
    ws.protocols(["kiwi"]).on_upgrade(move |socket| handle_socket(socket, params))
}

async fn handle_socket(
    mut socket: WebSocket,
    params: WebSocketParams,
) {
    let encoding = params.encoding.unwrap_or_default().clone();
    let run_socket_result = run_websocket(&mut socket, encoding).await;
    if let Err(report) = run_socket_result {
        let _ = send_message(encoding, &mut socket, &WsPacketServerbound::ErrorClose(report)).await;
    }
    let _ = socket.close().await;
}

error_object!(SocketError, "Failed to handle socket");
async fn run_websocket(
    socket: &mut WebSocket,
    encoding: Encoding,
) -> error_stack::Result<(), SocketError> {
    SocketSession::run(socket, encoding, |mut session| async move {
        loop {
            let message = session.receive().await
                .change_context(RunSessionError)?;
            match message {
                WsPacketClientbound::Echo(message) => {
                    session.send(&WsPacketServerbound::Hello(message)).await
                        .change_context(RunSessionError)?;
                }
            }
        }
    }).await.change_context(SocketError)?;

    Ok(())
}
