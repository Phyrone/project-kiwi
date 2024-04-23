use error_stack::Report;
use serde::{Deserialize, Serialize};

use crate::SocketError;

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WsPacketServerbound {
    #[serde(rename = "a_hello")]
    Hello(String),
    ErrorClose(Report<SocketError>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WsPacketClientbound {
    Echo(String),
    SubscribeMessages(WsPacketClientboundSubscribeMessages),
}

#[derive(Debug, Deserialize)]
pub struct WsPacketClientboundSubscribeMessages {
    pub channel: u64,
}