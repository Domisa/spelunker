mod stats;
mod model;
mod simulation;

use model::load_config;
use clap::Parser;
use model::{build_dag};
use simulation::parallel_num_trials;
use stats::{data_form, to_json};

// This struct is used to parse command line arguments using the clap crate.
#[derive(Parser)]
struct Args {

    #[arg(short, long)]
    config: String,

    #[arg(short, long)]
    iterations: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // Parse command line arguments
    let args = Args::parse();
    let services = load_config(&args.config)?;

    // Build the directed acyclic graph (DAG) and service lookup from the loaded services
    let dag = build_dag(&services);

    // Run the simulation in parallel to get the number of trials
    let num_trails = parallel_num_trials(&dag, &services, args.iterations);
    
    // Format the data and convert it to JSON to display the results
    let data = data_form(&num_trails);

    let jsoned_data = to_json(&data);

    println!("{}", jsoned_data?);
    Ok(())
}
