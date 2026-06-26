use std::error::Error;
use serde::Deserialize;

//TODO: ServiceTarget is will soon be too large. Alter for better reading.
#[derive(Deserialize)]
pub struct ServiceTarget {
    pub service_name: String,
    pub failure_rate: f64,
    pub service_topography: Vec<String>,
    pub median_latency: f64,

    #[serde(default = "default_std_dev")]
    pub std_dev: f64,
}

fn default_std_dev() -> f64 {
    0.5
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

pub build_service_lookup(services: &ServiceList) -> {
    let mut lookup = Hashmap::new();
    for service in &services.service_list {
        lookup.insert(service.service_name.clone(), service);
    }
    lookup

}