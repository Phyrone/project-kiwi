use clap::Parser;
use log::LevelFilter;

enum Module {
    Session,
    Forwarder,
}

#[derive(Debug, Clone, Parser)]
#[clap(version)]
pub struct StartupParams {
    #[clap(short, long, default_value = "info", env = "LOG_LEVEL")]
    pub log_level: LevelFilter,
}
