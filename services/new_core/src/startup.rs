use crate::web::WebServerParams;
use clap::Parser;
use common::{AllowRootParams, LoggerParams};
use database::DatabaseParams;
use log::LevelFilter;

#[derive(Debug, Clone, Parser)]
#[clap(version)]
pub struct StartupParams {
    #[clap(flatten)]
    pub logger_params: LoggerParams,

    #[clap(flatten)]
    pub allow_root_params: AllowRootParams,

    #[clap(flatten)]
    pub database_params: DatabaseParams,

    #[clap(flatten)]
    pub web_server_params: WebServerParams,
}
