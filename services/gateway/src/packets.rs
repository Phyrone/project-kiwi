use error_stack::Report;
use serde::{Deserialize, Serialize};

use crate::{SocketError, WebSocketQueryParams};

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WsPacketServerbound {
    Debug(DebugData),
    #[serde(rename = "a_hello")]
    Hello(String),
    CriticalError(Report<SocketError>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WsPacketClientbound {
    Echo(String),
    SubscribeMessages(WsPacketClientboundSubscribeMessages),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DebugData {
    QueryParams(WebSocketQueryParams),
}

#[derive(Debug, Deserialize)]
pub struct WsPacketClientboundSubscribeMessages {
    pub channel: u64,
}
