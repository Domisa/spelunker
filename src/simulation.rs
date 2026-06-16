// This is where the Monte Carlo calculation happens
use rayon;
use crate::model::{TrialResult, ServiceList};
use std::collections::HashMap;
use rand_distr::{LogNormal, Distribution};

fn walk_dag(dag: &HashMap<String, Vec<String>>, service: &str, visited: &mut HashMap<String, TrialResult>) -> TrialResult {
    
    if let Some(cached) = visited.get(service) {
        return cached.clone();
    }
    
    if let Some(dependency) = dag.get(service) {
        for dep in dependency {
             let dep_result = walk_dag(dag, dep, visited);
             if dep_result.failure {
                return TrialResult {failure: true, latency: 0.0};
             }
        }
       
    }

    todo!();
}

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
