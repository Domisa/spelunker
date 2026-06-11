use model::load_config;
use clap::Parser;

mod stats;
mod model;
mod simulation;

#[derive(Parser)]
struct Args {

    #[arg(short, long)]
    config: String,

    #[arg(short, long)]
    iterations: u64,
}

fn main() {
    
    println!("Hello, world!");
}
