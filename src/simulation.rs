// This is where the Monte Carlo calculation happens
use rayon;
use crate::model::{TrialResult, ServiceList};
use std::collections::HashMap;

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

