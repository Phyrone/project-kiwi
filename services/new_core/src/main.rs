use clap::Parser;
use error_stack::ResultExt;

use common::{error_object, init_logger};

use crate::startup::StartupParams;

mod startup;

error_object!(ApplicationError, "an error occurred in the application");

#[tokio::main]
async fn main() -> error_stack::Result<(), ApplicationError> {
    let params = StartupParams::parse();
    init_logger(params.log_level).change_context(ApplicationError)?;

    Ok(())
}
