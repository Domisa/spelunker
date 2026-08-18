// This is where the Monte Carlo calculation happens
use rayon::prelude::*;
use crate::model::{TrialResult, ServiceList, ServiceTarget, build_service_lookup};
use std::collections::HashMap;
use rand_distr::{LogNormal, Distribution};


fn sample_failure(failure_rate: f64) -> bool {
    let n = rand::random::<f64>();

    n < failure_rate
}


fn sample_latency(median_latency: f64, std_dev: f64) -> f64 {

    //replace unwrap for better error handling later
    let mu = median_latency.ln();
    let dist = LogNormal::new(mu, std_dev).unwrap();
    let latency = dist.sample(&mut rand::rng());
    latency
}

fn walk_dag(dag: &HashMap<String, Vec<String>>, service: &str, visited: &mut HashMap<String, TrialResult>, lookup: &HashMap<String, &ServiceTarget>) -> TrialResult {
    
    if let Some(cached) = visited.get(service) {
        return cached.clone();
    }
    
    if let Some(dependency) = dag.get(service) {
        for dep in dependency {
             let dep_result = walk_dag(dag, dep, visited, lookup);
             if dep_result.failure {
                return TrialResult {failure: true, latency: 0.0};
             }
        }
       
    }

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



fn trial_run(dag: &HashMap<String, Vec<String>>, services: &ServiceList) -> TrialResult {
    
    let mut visited = HashMap::new();
    let lookup = build_service_lookup(services);
    let result = walk_dag(dag, &services.entry_point, &mut visited, &lookup);
    result
    
}

fn parallel_num_trials(dag: &HashMap<String, Vec<String>>, services: &ServiceList, num_trials: u32) -> Vec<TrialResult> {
    let all_trial_results = (0..num_trials)
    .into_par_iter()
    .map(|_| trial_run(dag, services))
    .collect();
    all_trial_results

}