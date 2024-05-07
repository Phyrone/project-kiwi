use clap::Parser;
use common::{error_object, init_logger};
use error_stack::ResultExt;
use log::debug;

mod startup;

error_object!(ApplicationError, "Failed to start relay");

#[tokio::main]
async fn main() -> error_stack::Result<(), ApplicationError> {
    let params = startup::StartupParams::parse();
    init_logger(&params.logger_params).change_context(ApplicationError)?;
    debug!("params: {:#?}", params);

    Ok(())
}
