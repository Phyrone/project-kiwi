use tracing::{info, instrument};

use common::{with_bootstrap, Error};

use crate::startup::StartupParams;

mod startup;

with_bootstrap!(app_main, StartupParams);

#[derive(Debug, Error)]
#[error("failed to run relay")]
pub struct ApplicationError;

#[instrument]
async fn app_main(params: StartupParams) -> error_stack::Result<(), ApplicationError> {
    info!("Starting relay...");

    Ok(())
}
