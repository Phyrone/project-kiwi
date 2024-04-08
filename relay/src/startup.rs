use clap::Parser;


enum Module{
    Session,
    Forwarder,
}

#[derive(Debug, Clone)]
#[derive(Parser)]
#[clap(version)]
pub struct StartupParams {
    
}