use std::cmp::max;
use std::fmt::{Debug, Display};
use std::future::Future;
use std::time::Duration;

use clap::Parser;
use clap::{Args, CommandFactory, FromArgMatches};
use colored::{Color, Colorize};
pub use error_stack;
use error_stack::{Context, ResultExt};
use figlet_rs::FIGfont;
use human_panic::Metadata;
use is_root::is_root;
use itertools::Itertools;
use sysinfo::System;
pub use thiserror::Error;
use tracing::{info, instrument};

use crate::params::ParamsWithRequirements;

mod glide_id;
pub mod params;
pub mod tracing_logger;
pub mod utils;

#[derive(Debug, Error)]
pub enum BootstrapError {
    #[error("cannot initialize logger")]
    InitLogger,
    #[error("could not create tokio runtime")]
    CreateRuntime,
    #[error("cannot close logger")]
    CloseLogger,
    #[error("an error occurred while running the application")]
    RunApplication,
}
#[instrument(level = "trace", skip(f))]
pub fn run_bootstrap<E, F, R, PO, P>(
    app_name: &'static str,
    f: F,
) -> error_stack::Result<(), BootstrapError>
where
    E: Context,
    P: Args + Debug,
    PO: ParamsWithRequirements<P>,
    R: Future<Output = error_stack::Result<(), E>>,
    F: FnOnce(P) -> R,
{
    pre_boot(app_name);
    let params = PO::parse();
    prohibit_root_step(params.prohibit_root_params());
    print_startup_banner();
    tracing_logger::init_logger(params.logger_params())
        .change_context(BootstrapError::InitLogger)?;

    //TODO make runtime configurable
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        //using the strategy of kotlins io dispatcher here
        // see https://kotlinlang.org/api/kotlinx.coroutines/kotlinx-coroutines-core/kotlinx.coroutines/-dispatchers/-i-o.html
        .max_blocking_threads(max(64, num_cpus::get()))
        //the same here but with kotlins default dispatcher (almost tokios default as well)
        // see https://kotlinlang.org/api/kotlinx.coroutines/kotlinx-coroutines-core/kotlinx.coroutines/-dispatchers/-default.html
        .worker_threads(max(2, num_cpus::get()))
        //16mib stack size
        .thread_stack_size(16 * 1024 * 1024)
        .thread_name("async-worker")
        .thread_keep_alive(Duration::from_secs(30))
        .build()
        .change_context(BootstrapError::CreateRuntime)?;

    runtime
        .block_on(f(params.inner()))
        .change_context(BootstrapError::RunApplication)?;
    info!("Bye");
    Ok(())
}

fn pre_boot(app_name: &'static str) {
    human_panic::setup_panic!(
        Metadata::new(app_name, env!("CARGO_PKG_VERSION")).authors("Phyrone <phyrone@phyrone.de>")
    );
    dotenv::dotenv().ok();
}

fn prohibit_root_step(allow_root_params: &AllowRootParams) {
    if is_root() && !allow_root_params.allow_root {
        print_banner("Root is Evil", Color::Red);
        eprintln!("Running this app root is a bad idea. If you know what you are doing you can use the '--imRunningAsRootItIsEvilAndIKnowIt' option to run as root anyways.");
        std::process::exit(1);
    }
}

fn print_startup_banner() {
    let banner_printed = print_banner("Proj.-Kiwi", Color::Cyan);
    if banner_printed && sysinfo::IS_SUPPORTED_SYSTEM {
        let mut sys_info = System::new_all();
        sys_info.refresh_cpu_all();
        println!("  OS: {}", os_info::get());
        if let Some(kernel_version) = System::kernel_version() {
            println!("    Kernel: {}", kernel_version);
        }

        let process = sysinfo::get_current_pid()
            .ok()
            .map(|pid| sys_info.process(pid))
            .flatten();
        if let Some(process) = process {
            println!("  Process: ",);
            println!("    PID: {}", process.pid());
        }
        println!();
        let cpu_data = sys_info
            .cpus()
            .iter()
            .map(|cpu| cpu.brand())
            .sorted_unstable()
            .dedup_with_count()
            .collect::<Vec<_>>();
        for (count, cpu) in cpu_data {
            println!("  CPU: {}x {}", count, cpu.color(Color::Red).bold());
        }

        //println!("  CPU: {}", "Unknown".color(Color::Red).bold());

        let cpus_poll = sys_info.cpus();

        //Print Memory info
        sys_info.refresh_memory();
        let memory = sys_info.total_memory();
        println!(
            "  Memory: {}",
            humansize::format_size(memory, humansize::BINARY)
                .color(Color::Cyan)
                .bold()
        );

        println!()
    }
}

pub fn print_banner(text: &str, color: Color) -> bool {
    let font = FIGfont::standard();
    if let Ok(font) = font {
        let converted = font.convert(text);
        if let Some(converted) = converted {
            let converted = converted.to_string().color(color);
            println!("{}", converted);
            return true;
        }
    }
    false
}

#[derive(Debug, Clone, Args)]
pub struct AllowRootParams {
    #[clap(
        long = "imRunningAsRootItIsEvilAndIKnowIt",
        default_value = "false",
        hide = true
    )]
    pub allow_root: bool,
}

#[macro_use]
pub mod bootstrap {
    pub use clap;
    pub use error_stack;
    pub use jemallocator::Jemalloc;

    pub use crate::params::ParamsWithRequirements;
    pub use crate::run_bootstrap;
    pub use crate::BootstrapError;
    pub use crate::{tracing_logger::LoggerParams, AllowRootParams};

    #[macro_export]
    macro_rules! with_bootstrap {
        ($bootstrap_fn:expr,$params:ident) => {
            pub use common::bootstrap::run_bootstrap;
            #[cfg(not(target_env = "msvc"))]
            pub use common::bootstrap::Jemalloc;

            #[cfg(not(target_env = "msvc"))]
            #[global_allocator]
            static GLOBAL: Jemalloc = Jemalloc;

            #[derive(Debug, common::bootstrap::clap::Parser)]
            #[clap(
                version,
                rename_all_env = "SCREAMING_SNAKE_CASE",
                author = "Phyrone <phyrone@phyrone.de>"
            )]
            struct AppParams {
                #[clap(flatten)]
                allow_root_params: common::bootstrap::AllowRootParams,
                #[clap(flatten)]
                logger_params: common::bootstrap::LoggerParams,
                #[clap(flatten)]
                app_params: $params,
            }

            impl common::bootstrap::ParamsWithRequirements<$params> for AppParams {
                #[inline]
                fn prohibit_root_params(&self) -> &common::AllowRootParams {
                    &self.allow_root_params
                }

                #[inline]
                fn logger_params(&self) -> &common::bootstrap::LoggerParams {
                    &self.logger_params
                }

                #[inline]
                fn inner(self) -> $params {
                    self.app_params
                }
            }

            fn main(
            ) -> common::bootstrap::error_stack::Result<(), common::bootstrap::BootstrapError> {
                return run_bootstrap::<_, _, _, AppParams, $params>(
                    env!("CARGO_PKG_NAME"),
                    $bootstrap_fn,
                );
            }
        };
    }
}

#[macro_use]
pub mod error {
    #[macro_export]
    macro_rules! error_object {
        ($type:ident,$msg:literal) => {
            #[derive(Debug, common::Error)]
            #[error($msg)]
            pub struct $type;
        };
    }
}
