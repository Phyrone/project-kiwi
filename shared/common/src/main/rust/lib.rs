use clap::Args;
use colored::{Color, Colorize};
pub use error_stack;
use error_stack::ResultExt;
use fast_log::consts::LogSize;
use fast_log::Logger;
use fast_log::plugin::file_split::RollingType;
use fast_log::plugin::packer::GZipPacker;
use figlet_rs::FIGfont;
use human_panic::Metadata;
use is_root::is_root;
use log::{error, info, LevelFilter};

pub fn pre_boot(app_name: &'static str) {
    human_panic::setup_panic!(
        Metadata::new(app_name, env!("CARGO_PKG_VERSION")).authors("Phyrone <phyrone@phyrone.de>")
    );
    dotenv::dotenv().ok();
}

pub fn prohibit_root_step(allow_root_params: &AllowRootParams) {
    if is_root() && !allow_root_params.allow_root {
        print_banner("Root is Evil", Color::Red);
        eprintln!("Running this app root is a bad idea. If you know what you are doing you can use the '--imRunningAsRootItIsEvilAndIKnowIt' option to run as root anyways.");
        std::process::exit(1);
    }
}


pub fn startup_info_banner() -> bool {
    print_banner("Kiwi", Color::Cyan)
}


pub fn print_banner(
    text: &str,
    color: Color,
) -> bool {
    let font = FIGfont::standard();
    if let Ok(font) = font {
        let converted = font.convert(text);
        if let Some(converted) = converted {
            let converted = converted.to_string().color(color).bold();
            println!("{}", converted);
            return true;
        }
    }
    false
}

error_object!(InitLoggerError, "Failed to initialize logger");
pub fn init_logger(
    logger_params: &LoggerParams,
) -> error_stack::Result<&'static Logger, InitLoggerError> {
    let log_level = logger_params.log_level;
    log::set_max_level(log_level);
    let config = fast_log::Config::new()
        .console()
        .chan_len(Some(10_000))
        .file_split("log/latest.log", LogSize::MB(16), RollingType::KeepNum(10), GZipPacker {})
        .level(log_level);

    let result = fast_log::init(config).change_context(InitLoggerError)?;

    let level_color = match log_level {
        LevelFilter::Off => Color::Black,
        LevelFilter::Error => Color::Red,
        LevelFilter::Warn => Color::Yellow,
        LevelFilter::Info => Color::Green,
        LevelFilter::Debug => Color::Cyan,
        LevelFilter::Trace => Color::White,
    };
    info!("logger initialized with level: {}", log_level.to_string().to_lowercase().color(level_color).bold());
    Ok(result)
}

error_object!(CloseLoggerError, "Failed to close logger");
pub fn close_logger() -> error_stack::Result<(), CloseLoggerError> {
    fast_log::flush().change_context(CloseLoggerError)?;
    fast_log::exit().change_context(CloseLoggerError)?;
    Ok(())
}

#[derive(Debug, Clone, Args)]
#[group(id = "logger")]
pub struct LoggerParams {
    #[clap(short, long, default_value = "info", env = "LOG_LEVEL")]
    pub log_level: LevelFilter,
}

#[derive(Debug, Clone, Args)]
pub struct AllowRootParams {
    #[clap(long = "imRunningAsRootItIsEvilAndIKnowIt", default_value = "false", hide = true)]
    pub allow_root: bool,
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
