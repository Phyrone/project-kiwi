use error_stack::ResultExt;
use fast_log::Logger;

pub use error_stack;

error_object!(InitLoggerError, "Failed to initialize logger");
pub fn init_logger(
    log_level: log::LevelFilter,
) -> error_stack::Result<&'static Logger, InitLoggerError> {
    let config = fast_log::Config::new()
        .console()
        .chan_len(Some(1_000_000))
        .level(log_level);
    fast_log::init(config).change_context(InitLoggerError)
}

#[macro_use]
pub mod error_macro {
    #[macro_export]
    macro_rules! error_object {
        ($name:ident,$msg:expr) => {
            #[derive(Debug)]
            pub struct $name;
            impl std::fmt::Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, $msg)
                }
            }
            impl std::error::Error for $name {}
        };
    }
}
