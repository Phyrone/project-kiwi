use clap::Parser;
use error_stack::ResultExt;

use common::{
    close_logger, error_object, init_logger, pre_boot, prohibit_root_step, startup_info_banner,
};
use database::init_database;

use crate::startup::StartupParams;

mod startup;
mod web;

error_object!(ApplicationError, "an error occurred in the application");

fn main() -> error_stack::Result<(), ApplicationError> {
    pre_boot("new_core");
    let params = StartupParams::parse();
    prohibit_root_step(&params.allow_root_params);
    init_logger(&params.logger_params).change_context(ApplicationError)?;

    startup_info_banner();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .change_context(ApplicationError)?;
    let result = runtime.block_on(server_main(params));
    close_logger().change_context(ApplicationError)?;
    runtime.shutdown_background();
    result.change_context(ApplicationError)?;
    Ok(())
}

error_object!(ServerRunError, "an error occurred in the server_main");
async fn server_main(params: StartupParams) -> error_stack::Result<(), ServerRunError> {
    let database = init_database(&params.database_params)
        .await
        .change_context(ServerRunError)?;

    web::run_web_server(params.web_server_params, database)
        .await
        .change_context(ServerRunError)?;

    Ok(())
}
