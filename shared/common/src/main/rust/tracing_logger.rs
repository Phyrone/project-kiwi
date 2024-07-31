use clap::{Args, ValueEnum};
use error_stack::ResultExt;
use thiserror::Error;
use tracing::dispatcher::SetGlobalDefaultError;
use tracing::info;
use tracing::level_filters::LevelFilter;
use tracing_log::LogTracer;

#[derive(Debug, Args)]
pub struct LoggerParams {
    #[clap(long, short, default_value = "info", env = "LOG_LEVEL")]
    #[cfg_attr(debug_assertions, clap(default_value = "debug"))]
    pub log_level: LevelFilter,

    #[clap(long, default_value = "compact", env = "LOG_FORMAT")]
    //#[cfg_attr(debug_assertions, clap(default_value = "pretty"))]
    pub log_format: LoggerFormat,

    #[clap(long, default_value = "false", env = "LOG_SOURCE_LOCATION")]
    pub log_source_location: bool,

    #[clap(long, default_value = "false", env = "LOG_USE_TOKIO_CONSOLE")]
    pub with_tokio_console: bool,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum LoggerFormat {
    /// A compact format that displays the most relevant information in a single line.
    Compact,
    /// A pretty format that displays the most relevant information in a human-readable format.
    ///  It uses multiple lines and displayes the orgin of the log message.
    ///  Useful for debugging but produces a lot of output.
    Pretty,
    /// A json format that displays the most relevant information in a json format.
    /// Useful for automated processing of logs.
    Json,
}
#[derive(Error, Debug)]
pub enum InitLoggerError {
    #[error("cannot set logger as global default")]
    SetGlobalDefault,
}

pub(crate) fn init_logger(
    logger_params: &LoggerParams,
) -> error_stack::Result<(), InitLoggerError> {
    LogTracer::init().expect("Failed to set logger");

    if logger_params.with_tokio_console {
        console_subscriber::init()
    } else {
        let subscriber = tracing_subscriber::fmt::Subscriber::builder()
            .with_max_level(logger_params.log_level)
            .with_ansi(true)
            .log_internal_errors(true)
            .with_target(false)
            .with_file(logger_params.log_source_location)
            .with_line_number(logger_params.log_source_location)
            .with_thread_ids(false)
            .with_thread_names(true)
            .with_level(true);

        match logger_params.log_format {
            LoggerFormat::Compact => {
                tracing::subscriber::set_global_default(subscriber.compact().finish())
            }

            LoggerFormat::Pretty => {
                tracing::subscriber::set_global_default(subscriber.pretty().finish())
            }

            LoggerFormat::Json => {
                tracing::subscriber::set_global_default(subscriber.json().finish())
            }
        }
        .change_context(InitLoggerError::SetGlobalDefault)?;
        info!("Logger initialized with level: {}", logger_params.log_level);
    }
    Ok(())
}
