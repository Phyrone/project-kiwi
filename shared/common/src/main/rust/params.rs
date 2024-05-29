use std::fmt::Debug;

use clap::{Args, Parser};

use crate::tracing_logger::LoggerParams;
use crate::AllowRootParams;

pub trait ParamsWithRequirements<T>: Parser + Debug
where
    T: Args + Debug,
{
    fn prohibit_root_params(&self) -> &AllowRootParams;
    fn logger_params(&self) -> &LoggerParams;
    fn inner(self) -> T;
}
