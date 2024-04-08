use clap::Parser;

enum Module {
    Session,
    Forwarder,
}

#[derive(Debug, Clone, Parser)]
#[clap(version)]
pub struct StartupParams {}
