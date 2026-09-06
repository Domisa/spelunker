use crate::model::TrialResult;
use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
    pub failure_rate: f64,
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

fn latency_percentiles(results: &Vec<TrialResult>) -> (f64, f64, f64) {
    let mut sorted: Vec<f64> = results.iter().map(|r| r.latency).collect();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let len = sorted.len();
    (sorted[len * 50/100], sorted[len * 95/100], sorted[len * 99/100])
}

pub fn data_form(results: &Vec<TrialResult>) -> Data {
    let failure_rate: f64 = failure_rate(results);
    let availability: f64 = availability(results);
    let latency_percentiles: (f64, f64, f64) = latency_percentiles(results);
    Data {failure_rate, availability, p50: latency_percentiles.0, p95: latency_percentiles.1, p99: latency_percentiles.2}
}

pub fn to_json(data: &Data) -> Result<String, serde_json::Error> {
    serde_json::to_string(data)
}