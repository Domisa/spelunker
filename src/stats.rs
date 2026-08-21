use crate::model::TrialResult;

pub struct Data {
    pub failure_rate: f64,
    pub latency: f64,
    pub availability: f64,
    pub p50: f64,
    pub p95: f64,
    pub p99: f64,
}

fn failure_rate(results: &Vec<TrialResult>) -> f64 {
    let mut fail_count: u32 = 0;
    for result in results {
        if result.failure {
        fail_count += 1;
        }
    }
    fail_count as f64/results.len() as f64
}

fn availability(results: &Vec<TrialResult>) -> f64 {
    1.0 - failure_rate(results)
}

