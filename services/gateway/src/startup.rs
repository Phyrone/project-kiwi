use std::net::SocketAddr;

use clap::Parser;
use log::LevelFilter;

#[derive(Debug, Parser, Clone)]
#[clap(version)]
pub struct StartupParams {
    #[clap(short, long, default_value = "info", env = "LOG_LEVEL")]
    pub log_level: LevelFilter,

    #[clap(short, long, default_value = "0.0.0.0:8080", env = "BIND")]
    pub bind: SocketAddr,
}

