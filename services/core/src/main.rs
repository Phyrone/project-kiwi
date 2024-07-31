use clap::Parser;
use error_stack::{FutureExt, ResultExt};
use tokio::{join, try_join};
use tokio_util::sync::CancellationToken;
use tracing::{info, instrument};

use common::{with_bootstrap, Error};
use database::init_database;

use crate::startup::StartupParams;

mod graphql;
mod startup;
mod web;

with_bootstrap!(server_main, StartupParams);

#[derive(Debug, Clone, Error)]
enum CoreAppError {
    #[error("initializing the database failed")]
    InitDatabase,
    #[error("an error occured while running the web server")]
    WebServer,
}

#[instrument(level = "trace")]
async fn server_main(params: StartupParams) -> error_stack::Result<(), CoreAppError> {
    let database = init_database(&params.database_params)
        .await
        .change_context(CoreAppError::InitDatabase)?;

    let shutdown_token = CancellationToken::new();
    let shutdown_token_for_user_interrupt = shutdown_token.clone();
    tokio::spawn(async move {
        let shutdown_token = shutdown_token_for_user_interrupt;
        let user_interrupt = tokio::signal::ctrl_c();
        tokio::select! {
            _ = user_interrupt => {
                info!("Received user interrupt, shutting down");
                shutdown_token.cancel();
            }
            _ = shutdown_token.cancelled() => {}
        }
    });
    let web_server_task =
        web::run_web_server(params.web_server_params, database, shutdown_token.clone());
    let (web_server_task,) = join!(web_server_task);
    web_server_task.change_context(CoreAppError::WebServer)?;

    Ok(())
}
