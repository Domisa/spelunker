// This is where the Monte Carlo calculation happens
use rayon::prelude::*;
use crate::model::{TrialResult, ServiceList, ServiceTarget, build_service_lookup};
use std::collections::HashMap;
use rand_distr::{LogNormal, Distribution};


// This function samples a failure based on the provided failure rate, returning true if a failure occurs and false otherwise.
fn sample_failure(failure_rate: f64) -> bool {
    let n = rand::random::<f64>();

    n < failure_rate
}

// This function samples latency based on a log-normal distribution using the provided median latency and standard deviation.
fn sample_latency(median_latency: f64, std_dev: f64) -> f64 {

    //replace unwrap for better error handling later
    let mu = median_latency.ln();
    let dist = LogNormal::new(mu, std_dev).unwrap();
    let latency = dist.sample(&mut rand::rng());
    latency
}

// This function recursively walks through the directed acyclic graph (DAG) of services, checking for failures and calculating latency for each service and its dependencies.
fn walk_dag(dag: &HashMap<String, Vec<String>>, service: &str, visited: &mut HashMap<String, TrialResult>, lookup: &HashMap<String, &ServiceTarget>) -> TrialResult {
    
    // Check if the service has already been visited to avoid redundant calculations and potential infinite loops.
    if let Some(cached) = visited.get(service) {
        return cached.clone();
    }
    
    // If the service has dependencies, recursively walk through each dependency and check for failures. 
    // If any dependency fails, return a failure result for the current service.
    if let Some(dependency) = dag.get(service) {
        for dep in dependency {
             let dep_result = walk_dag(dag, dep, visited, lookup);
             if dep_result.failure {
                return TrialResult {failure: true, latency: 0.0};
             }
        }
       
    }

    // If the service has not been visited and has no failing dependencies, sample its failure and latency based on its properties.
    if let Some(target) = lookup.get(service) {
        let failure = sample_failure(target.failure_rate);
        let latency = sample_latency(target.median_latency, target.std_dev);
        let result = TrialResult { failure, latency };
        visited.insert(service.to_string(), result.clone());
        result
    } else {
        TrialResult { failure: true, latency: 0.0 }
    }
}


// This function runs a single trial of the simulation, starting from the entry point of the DAG and returning the result of the trial.
fn trial_run(dag: &HashMap<String, Vec<String>>, services: &ServiceList) -> TrialResult {
    
    let mut visited = HashMap::new();
    let lookup = build_service_lookup(services);
    let result = walk_dag(dag, &services.entry_point, &mut visited, &lookup);
    result
    
}

// This function runs multiple trials of the simulation in parallel, returning a vector of TrialResult for each trial.
pub fn parallel_num_trials(dag: &HashMap<String, Vec<String>>, services: &ServiceList, num_trials: u32) -> Vec<TrialResult> {
    let all_trial_results = (0..num_trials)
    .into_par_iter()
    .map(|_| trial_run(dag, services))
    .collect();
    all_trial_results

}