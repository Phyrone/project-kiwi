use clap::Parser;
use error_stack::ResultExt;
use tracing::instrument;

use common::{with_bootstrap, Error};
use database::init_database;

use crate::startup::StartupParams;

mod startup;
mod web;
mod graphql;

with_bootstrap!(server_main, StartupParams);

#[derive(Debug, Clone, Error)]
enum CoreAppError {
    #[error("an error occured while connecting to the database")]
    InitDatabase,
    #[error("an error occured while running the web server")]
    WebServer,
}

#[instrument(level = "trace")]
async fn server_main(params: StartupParams) -> error_stack::Result<(), CoreAppError> {
    let database = init_database(&params.database_params)
        .await
        .change_context(CoreAppError::InitDatabase)?;

    web::run_web_server(params.web_server_params, database)
        .await
        .change_context(CoreAppError::WebServer)?;

    Ok(())
}
