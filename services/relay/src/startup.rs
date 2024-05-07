use clap::Parser;
use common::LoggerParams;
use log::LevelFilter;

enum Module {
    Session,
    Forwarder,
}

#[derive(Debug, Clone, Parser)]
#[clap(version)]
pub struct StartupParams {
    #[clap(flatten)]
    pub logger_params: LoggerParams,
}
