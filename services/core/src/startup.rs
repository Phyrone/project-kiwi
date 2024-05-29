use clap::Parser;

use database::DatabaseParams;

use crate::web::WebServerParams;

#[derive(Debug, Clone, Parser)]
#[clap(version)]
pub struct StartupParams {
    #[clap(flatten)]
    pub database_params: DatabaseParams,

    #[clap(flatten)]
    pub web_server_params: WebServerParams,
}
