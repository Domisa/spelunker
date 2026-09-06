mod stats;
mod model;
mod simulation;

use model::load_config;
use clap::Parser;
use model::{build_dag, build_service_lookup};
use simulation::parallel_num_trials;
use stats::{data_form, to_json};

#[derive(Parser)]
struct Args {

    #[arg(short, long)]
    config: String,

    #[arg(short, long)]
    iterations: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let services = load_config(&args.config)?;

    let dag = build_dag(&services);
    let service_lookup = build_service_lookup(&services);

    let num_trails = parallel_num_trials(&dag, &services, args.iterations);
    
    let data = data_form(&num_trails);

    let jsoned_data = to_json(&data);

    println!("{}", jsoned_data?);
    Ok(())
}
