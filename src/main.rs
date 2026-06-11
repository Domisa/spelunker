mod stats;
mod model;
mod simulation;

use model::load_config;
use clap::Parser;


#[derive(Parser)]
struct Args {

    #[arg(short, long)]
    config: String,

    #[arg(short, long)]
    iterations: u64,
}

fn main() {
    let args = Args::parse();
    println!("Hello, world!");
}
