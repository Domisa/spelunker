use std::error::Error;
use serde::Deserialize;

// This struct represents a service target with its associated properties.
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

// This struct represents a list of service targets and the entry point for the simulation.
#[derive(Deserialize)]
pub struct ServiceList {
    pub service_list: Vec<ServiceTarget>,
    pub entry_point: String,
    //pub num_trails: u32,
}

// This struct represents the result of a single trial in the simulation, including latency and failure status.
#[derive(Clone)]
pub struct TrialResult{
    pub latency: f64,
    pub failure: bool,
}

// This function loads the configuration from a TOML file and returns a ServiceList.
pub fn load_config (path: &str) -> Result<ServiceList, Box<dyn Error>>{
    let contents = std::fs::read_to_string(path)?;
    let config: ServiceList = toml::from_str(&contents)?;
    Ok(config)
}

// This function builds a directed acyclic graph (DAG) from the provided ServiceList, mapping service names to their topography.
pub fn build_dag (services: &ServiceList) -> std::collections::HashMap<String, Vec<String>> {
    let mut dag = std::collections::HashMap::new();
    for service in &services.service_list {
        dag.insert(service.service_name.clone(), service.service_topography.clone());
    }
    dag
}

// This function builds a lookup table for services, mapping service names to their corresponding ServiceTarget references.
pub fn build_service_lookup (services: &ServiceList) -> std::collections::HashMap<String, &ServiceTarget> {
    let mut lookup = std::collections::HashMap::new();
    for service in &services.service_list {
        lookup.insert(service.service_name.clone(), service);
    }
    lookup

}