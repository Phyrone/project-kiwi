#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use tracing::{info, instrument};
use common::{Error, with_bootstrap};
use crate::startup::StartupParams;
mod startup {
    use clap::Args;
    pub struct StartupParams {}
    #[automatically_derived]
    impl ::core::fmt::Debug for StartupParams {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "StartupParams")
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for StartupParams {
        #[inline]
        fn clone(&self) -> StartupParams {
            StartupParams {}
        }
    }
    #[allow(
        dead_code,
        unreachable_code,
        unused_variables,
        unused_braces,
        unused_qualifications,
    )]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting,
        clippy::almost_swapped,
        clippy::redundant_locals,
    )]
    #[automatically_derived]
    impl clap::FromArgMatches for StartupParams {
        fn from_arg_matches(
            __clap_arg_matches: &clap::ArgMatches,
        ) -> ::std::result::Result<Self, clap::Error> {
            Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
        }
        fn from_arg_matches_mut(
            __clap_arg_matches: &mut clap::ArgMatches,
        ) -> ::std::result::Result<Self, clap::Error> {
            #![allow(deprecated)]
            let v = StartupParams {};
            ::std::result::Result::Ok(v)
        }
        fn update_from_arg_matches(
            &mut self,
            __clap_arg_matches: &clap::ArgMatches,
        ) -> ::std::result::Result<(), clap::Error> {
            self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
        }
        fn update_from_arg_matches_mut(
            &mut self,
            __clap_arg_matches: &mut clap::ArgMatches,
        ) -> ::std::result::Result<(), clap::Error> {
            #![allow(deprecated)]
            ::std::result::Result::Ok(())
        }
    }
    #[allow(
        dead_code,
        unreachable_code,
        unused_variables,
        unused_braces,
        unused_qualifications,
    )]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting,
        clippy::almost_swapped,
        clippy::redundant_locals,
    )]
    #[automatically_derived]
    impl clap::Args for StartupParams {
        fn group_id() -> Option<clap::Id> {
            Some(clap::Id::from("StartupParams"))
        }
        fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
            {
                let __clap_app = __clap_app
                    .group(
                        clap::ArgGroup::new("StartupParams")
                            .multiple(true)
                            .args({
                                let members: [clap::Id; 0usize] = [];
                                members
                            }),
                    );
                __clap_app
            }
        }
        fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
            {
                let __clap_app = __clap_app
                    .group(
                        clap::ArgGroup::new("StartupParams")
                            .multiple(true)
                            .args({
                                let members: [clap::Id; 0usize] = [];
                                members
                            }),
                    );
                __clap_app
            }
        }
    }
}
pub use common::bootstrap::run_bootstrap;
#[cfg(not(target_env = "msvc"))]
pub use common::bootstrap::Jemalloc;
#[cfg(not(target_env = "msvc"))]
static GLOBAL: Jemalloc = Jemalloc;
const _: () = {
    #[rustc_std_internal_symbol]
    unsafe fn __rust_alloc(size: usize, align: usize) -> *mut u8 {
        ::core::alloc::GlobalAlloc::alloc(
            &GLOBAL,
            ::core::alloc::Layout::from_size_align_unchecked(size, align),
        )
    }
    #[rustc_std_internal_symbol]
    unsafe fn __rust_dealloc(ptr: *mut u8, size: usize, align: usize) -> () {
        ::core::alloc::GlobalAlloc::dealloc(
            &GLOBAL,
            ptr,
            ::core::alloc::Layout::from_size_align_unchecked(size, align),
        )
    }
    #[rustc_std_internal_symbol]
    unsafe fn __rust_realloc(
        ptr: *mut u8,
        size: usize,
        align: usize,
        new_size: usize,
    ) -> *mut u8 {
        ::core::alloc::GlobalAlloc::realloc(
            &GLOBAL,
            ptr,
            ::core::alloc::Layout::from_size_align_unchecked(size, align),
            new_size,
        )
    }
    #[rustc_std_internal_symbol]
    unsafe fn __rust_alloc_zeroed(size: usize, align: usize) -> *mut u8 {
        ::core::alloc::GlobalAlloc::alloc_zeroed(
            &GLOBAL,
            ::core::alloc::Layout::from_size_align_unchecked(size, align),
        )
    }
};
#[clap(
    version,
    rename_all_env = "SCREAMING_SNAKE_CASE",
    author = "Phyrone <phyrone@phyrone.de>",
)]
struct AppParams {
    #[clap(flatten)]
    allow_root_params: common::bootstrap::AllowRootParams,
    #[clap(flatten)]
    logger_params: common::bootstrap::LoggerParams,
    #[clap(flatten)]
    app_params: StartupParams,
}
#[automatically_derived]
impl ::core::fmt::Debug for AppParams {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(
            f,
            "AppParams",
            "allow_root_params",
            &self.allow_root_params,
            "logger_params",
            &self.logger_params,
            "app_params",
            &&self.app_params,
        )
    }
}
#[automatically_derived]
#[allow(unused_qualifications, clippy::redundant_locals)]
impl clap::Parser for AppParams {}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
    clippy::redundant_locals,
)]
#[automatically_derived]
impl clap::CommandFactory for AppParams {
    fn command<'b>() -> clap::Command {
        let __clap_app = clap::Command::new("relay");
        <Self as clap::Args>::augment_args(__clap_app)
    }
    fn command_for_update<'b>() -> clap::Command {
        let __clap_app = clap::Command::new("relay");
        <Self as clap::Args>::augment_args_for_update(__clap_app)
    }
}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
    clippy::redundant_locals,
)]
#[automatically_derived]
impl clap::FromArgMatches for AppParams {
    fn from_arg_matches(
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn from_arg_matches_mut(
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        #![allow(deprecated)]
        let v = AppParams {
            allow_root_params: <common::bootstrap::AllowRootParams as clap::FromArgMatches>::from_arg_matches_mut(
                __clap_arg_matches,
            )?,
            logger_params: <common::bootstrap::LoggerParams as clap::FromArgMatches>::from_arg_matches_mut(
                __clap_arg_matches,
            )?,
            app_params: <StartupParams as clap::FromArgMatches>::from_arg_matches_mut(
                __clap_arg_matches,
            )?,
        };
        ::std::result::Result::Ok(v)
    }
    fn update_from_arg_matches(
        &mut self,
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn update_from_arg_matches_mut(
        &mut self,
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        #![allow(deprecated)]
        {
            #[allow(non_snake_case)]
            let allow_root_params = &mut self.allow_root_params;
            <common::bootstrap::AllowRootParams as clap::FromArgMatches>::update_from_arg_matches_mut(
                allow_root_params,
                __clap_arg_matches,
            )?;
        }
        {
            #[allow(non_snake_case)]
            let logger_params = &mut self.logger_params;
            <common::bootstrap::LoggerParams as clap::FromArgMatches>::update_from_arg_matches_mut(
                logger_params,
                __clap_arg_matches,
            )?;
        }
        {
            #[allow(non_snake_case)]
            let app_params = &mut self.app_params;
            <StartupParams as clap::FromArgMatches>::update_from_arg_matches_mut(
                app_params,
                __clap_arg_matches,
            )?;
        }
        ::std::result::Result::Ok(())
    }
}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
    clippy::redundant_locals,
)]
#[automatically_derived]
impl clap::Args for AppParams {
    fn group_id() -> Option<clap::Id> {
        Some(clap::Id::from("AppParams"))
    }
    fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
        {
            let __clap_app = __clap_app
                .group(
                    clap::ArgGroup::new("AppParams")
                        .multiple(true)
                        .args({
                            let members: [clap::Id; 0] = [];
                            members
                        }),
                );
            let __clap_app = __clap_app;
            let __clap_app = <common::bootstrap::AllowRootParams as clap::Args>::augment_args(
                __clap_app,
            );
            let __clap_app = __clap_app;
            let __clap_app = <common::bootstrap::LoggerParams as clap::Args>::augment_args(
                __clap_app,
            );
            let __clap_app = __clap_app;
            let __clap_app = <StartupParams as clap::Args>::augment_args(__clap_app);
            __clap_app.version("0.1.0").author("Phyrone <phyrone@phyrone.de>")
        }
    }
    fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
        {
            let __clap_app = __clap_app
                .group(
                    clap::ArgGroup::new("AppParams")
                        .multiple(true)
                        .args({
                            let members: [clap::Id; 0] = [];
                            members
                        }),
                );
            let __clap_app = __clap_app;
            let __clap_app = <common::bootstrap::AllowRootParams as clap::Args>::augment_args_for_update(
                __clap_app,
            );
            let __clap_app = __clap_app;
            let __clap_app = <common::bootstrap::LoggerParams as clap::Args>::augment_args_for_update(
                __clap_app,
            );
            let __clap_app = __clap_app;
            let __clap_app = <StartupParams as clap::Args>::augment_args_for_update(
                __clap_app,
            );
            __clap_app.version("0.1.0").author("Phyrone <phyrone@phyrone.de>")
        }
    }
}
impl common::bootstrap::ParamsWithRequirements<StartupParams> for AppParams {
    #[inline]
    fn prohibit_root_params(&self) -> &common::AllowRootParams {
        &self.allow_root_params
    }
    #[inline]
    fn logger_params(&self) -> &common::bootstrap::LoggerParams {
        &self.logger_params
    }
    #[inline]
    fn inner(self) -> StartupParams {
        self.app_params
    }
}
fn main() -> common::bootstrap::error_stack::Result<
    (),
    common::bootstrap::BootstrapError,
> {
    (/*ERROR*/)
}
#[error("failed to run relay")]
pub struct ApplicationError;
#[automatically_derived]
impl ::core::fmt::Debug for ApplicationError {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "ApplicationError")
    }
}
#[allow(unused_qualifications)]
impl std::error::Error for ApplicationError {}
#[allow(unused_qualifications)]
impl ::core::fmt::Display for ApplicationError {
    #[allow(clippy::used_underscore_binding)]
    fn fmt(&self, __formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        #[allow(unused_variables, deprecated)]
        let Self {} = self;
        __formatter.write_str("failed to run relay")
    }
}
async fn app_main(params: StartupParams) -> error_stack::Result<(), ApplicationError> {
    {}
    let __tracing_attr_span = {
        use ::tracing::__macro_support::Callsite as _;
        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
            static META: ::tracing::Metadata<'static> = {
                ::tracing_core::metadata::Metadata::new(
                    "app_main",
                    "relay",
                    tracing::Level::INFO,
                    ::core::option::Option::Some("services/relay/src/main.rs"),
                    ::core::option::Option::Some(15u32),
                    ::core::option::Option::Some("relay"),
                    ::tracing_core::field::FieldSet::new(
                        &["params"],
                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                    ),
                    ::tracing::metadata::Kind::SPAN,
                )
            };
            ::tracing::callsite::DefaultCallsite::new(&META)
        };
        let mut interest = ::tracing::subscriber::Interest::never();
        if tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
            && tracing::Level::INFO <= ::tracing::level_filters::LevelFilter::current()
            && {
                interest = __CALLSITE.interest();
                !interest.is_never()
            }
            && ::tracing::__macro_support::__is_enabled(__CALLSITE.metadata(), interest)
        {
            let meta = __CALLSITE.metadata();
            ::tracing::Span::new(
                meta,
                &{
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = meta.fields().iter();
                    meta.fields()
                        .value_set(
                            &[
                                (
                                    &::core::iter::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::core::option::Option::Some(
                                        &tracing::field::debug(&params) as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                },
            )
        } else {
            let span = ::tracing::__macro_support::__disabled_span(
                __CALLSITE.metadata(),
            );
            if match tracing::Level::INFO {
                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                _ => ::tracing::log::Level::Trace,
            } <= ::tracing::log::STATIC_MAX_LEVEL
            {
                if !::tracing::dispatcher::has_been_set() {
                    {
                        span.record_all(
                            &{
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = __CALLSITE.metadata().fields().iter();
                                __CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &::core::iter::Iterator::next(&mut iter)
                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                ::core::option::Option::Some(
                                                    &tracing::field::debug(&params) as &dyn Value,
                                                ),
                                            ),
                                        ],
                                    )
                            },
                        );
                    }
                } else {
                    {}
                }
            } else {
                {}
            };
            span
        }
    };
    let __tracing_instrument_future = async move {
        #[allow(
            unknown_lints,
            unreachable_code,
            clippy::diverging_sub_expression,
            clippy::let_unit_value,
            clippy::unreachable,
            clippy::let_with_type_underscore,
            clippy::empty_loop
        )]
        if false {
            let __tracing_attr_fake_return: error_stack::Result<(), ApplicationError> = loop {};
            return __tracing_attr_fake_return;
        }
        {
            {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "event services/relay/src/main.rs:17",
                            "relay",
                            ::tracing::Level::INFO,
                            ::core::option::Option::Some("services/relay/src/main.rs"),
                            ::core::option::Option::Some(17u32),
                            ::core::option::Option::Some("relay"),
                            ::tracing_core::field::FieldSet::new(
                                &["message"],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::EVENT,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let enabled = ::tracing::Level::INFO
                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::INFO
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        let interest = __CALLSITE.interest();
                        !interest.is_never()
                            && ::tracing::__macro_support::__is_enabled(
                                __CALLSITE.metadata(),
                                interest,
                            )
                    };
                if enabled {
                    (|value_set: ::tracing::field::ValueSet| {
                        let meta = __CALLSITE.metadata();
                        ::tracing::Event::dispatch(meta, &value_set);
                        if match ::tracing::Level::INFO {
                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                            _ => ::tracing::log::Level::Trace,
                        } <= ::tracing::log::STATIC_MAX_LEVEL
                        {
                            if !::tracing::dispatcher::has_been_set() {
                                {
                                    use ::tracing::log;
                                    let level = match ::tracing::Level::INFO {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    };
                                    if level <= log::max_level() {
                                        let meta = __CALLSITE.metadata();
                                        let log_meta = log::Metadata::builder()
                                            .level(level)
                                            .target(meta.target())
                                            .build();
                                        let logger = log::logger();
                                        if logger.enabled(&log_meta) {
                                            ::tracing::__macro_support::__tracing_log(
                                                meta,
                                                logger,
                                                log_meta,
                                                &value_set,
                                            )
                                        }
                                    }
                                }
                            } else {
                                {}
                            }
                        } else {
                            {}
                        };
                    })({
                        #[allow(unused_imports)]
                        use ::tracing::field::{debug, display, Value};
                        let mut iter = __CALLSITE.metadata().fields().iter();
                        __CALLSITE
                            .metadata()
                            .fields()
                            .value_set(
                                &[
                                    (
                                        &::core::iter::Iterator::next(&mut iter)
                                            .expect("FieldSet corrupted (this is a bug)"),
                                        ::core::option::Option::Some(
                                            &format_args!("Starting relay...") as &dyn Value,
                                        ),
                                    ),
                                ],
                            )
                    });
                } else {
                    if match ::tracing::Level::INFO {
                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                        _ => ::tracing::log::Level::Trace,
                    } <= ::tracing::log::STATIC_MAX_LEVEL
                    {
                        if !::tracing::dispatcher::has_been_set() {
                            {
                                use ::tracing::log;
                                let level = match ::tracing::Level::INFO {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                };
                                if level <= log::max_level() {
                                    let meta = __CALLSITE.metadata();
                                    let log_meta = log::Metadata::builder()
                                        .level(level)
                                        .target(meta.target())
                                        .build();
                                    let logger = log::logger();
                                    if logger.enabled(&log_meta) {
                                        ::tracing::__macro_support::__tracing_log(
                                            meta,
                                            logger,
                                            log_meta,
                                            &{
                                                #[allow(unused_imports)]
                                                use ::tracing::field::{debug, display, Value};
                                                let mut iter = __CALLSITE.metadata().fields().iter();
                                                __CALLSITE
                                                    .metadata()
                                                    .fields()
                                                    .value_set(
                                                        &[
                                                            (
                                                                &::core::iter::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::core::option::Option::Some(
                                                                    &format_args!("Starting relay...") as &dyn Value,
                                                                ),
                                                            ),
                                                        ],
                                                    )
                                            },
                                        )
                                    }
                                }
                            }
                        } else {
                            {}
                        }
                    } else {
                        {}
                    };
                }
            };
            Ok(())
        }
    };
    if !__tracing_attr_span.is_disabled() {
        tracing::Instrument::instrument(__tracing_instrument_future, __tracing_attr_span)
            .await
    } else {
        __tracing_instrument_future.await
    }
}
