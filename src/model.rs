use std::error:Error;

#[derive(Deserialize)]
pub struct ServiceTarget {
    pub service_name: String,
    pub failure_rate: f64,
    pub service_topography: Vec<String>,
}

#[derive(Deserialize)]
pub struct ServiceList {
    pub service_list: Vec<ServiceTarget>,
    //pub num_trails: u32,
}

pub fn load_config (path: &str) -> Result<ServiceList, Box<dyn Error>>{
    let _data = std::fs::read_to_string(path);
    todo!();
}