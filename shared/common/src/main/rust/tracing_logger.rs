use clap::{Args, ValueEnum};
use tracing::dispatcher::SetGlobalDefaultError;
use tracing::level_filters::LevelFilter;
use tracing_log::LogTracer;

#[derive(Debug, Args)]
pub struct LoggerParams {
    #[clap(long, short, default_value = "info", env = "LOG_LEVEL")]
    #[cfg_attr(debug_assertions, clap(default_value = "debug"))]
    pub log_level: LevelFilter,

    #[clap(long, default_value = "compact", env = "LOG_FORMAT")]
    #[cfg_attr(debug_assertions, clap(default_value = "pretty"))]
    pub log_format: LoggerFormat,
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

pub(crate) fn init_logger(logger_params: &LoggerParams) -> Result<(), SetGlobalDefaultError> {
    LogTracer::init().expect("Failed to set logger");

    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        .with_max_level(logger_params.log_level)
        .with_ansi(true)
        .with_thread_ids(false)
        .with_thread_names(true);

    return match logger_params.log_format {
        LoggerFormat::Compact => {
            tracing::subscriber::set_global_default(subscriber.compact().finish())
        }

        LoggerFormat::Pretty => {
            tracing::subscriber::set_global_default(subscriber.pretty().finish())
        }

        LoggerFormat::Json => tracing::subscriber::set_global_default(subscriber.json().finish()),
    };
}
