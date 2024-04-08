mod startup;

use clap::Parser;
use proto::tonic;
use proto::tonic::{Request, Response, Status};

fn main() {
    
    let params = startup::StartupParams::parse();
    
    println!("params: {:#?}", params);
}

