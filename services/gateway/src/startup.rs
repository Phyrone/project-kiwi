use std::net::SocketAddr;

use clap::Parser;

use common::{AllowRootParams, LoggerParams};
use database::DatabaseParams;

#[derive(Debug, Parser, Clone)]
#[clap(version)]
pub struct StartupParams {
    #[clap(short, long, default_value = "0.0.0.0:8080", env = "BIND")]
    pub binds: Vec<SocketAddr>,

    #[clap(flatten)]
    pub database_params: DatabaseParams,

    #[clap(flatten)]
    pub allow_root_params: AllowRootParams,

    #[clap(flatten)]
    pub logger_params: LoggerParams,
}
