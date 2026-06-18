use std::error::Error;
use serde::Deserialize;

//TODO: ServiceTarget is will soon be too large. Alter for better reading.
#[derive(Deserialize)]
pub struct ServiceTarget {
    pub service_name: String,
    pub failure_rate: f64,
    pub service_topography: Vec<String>,
    pub median_latency: f64,
    pub std_dev: f64,
}

#[derive(Deserialize)]
pub struct ServiceList {
    pub service_list: Vec<ServiceTarget>,
    pub entry_point: String,
    //pub num_trails: u32,
}

#[derive(Clone)]
pub struct TrialResult{
    pub latency: f64,
    pub failure: bool,
}

pub fn load_config (path: &str) -> Result<ServiceList, Box<dyn Error>>{
    let contents = std::fs::read_to_string(path)?;
    let config: ServiceList = toml::from_str(&contents)?;
    Ok(config)
}
pub fn build_dag (services: &ServiceList) -> std::collections::HashMap<String, Vec<String>> {
    let mut dag = std::collections::HashMap::new();
    for service in &services.service_list {
        dag.insert(service.service_name.clone(), service.service_topography.clone());
    }
    dag
}