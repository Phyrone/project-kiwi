use clap::Parser;
use log::LevelFilter;
use common::LoggerParams;

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
