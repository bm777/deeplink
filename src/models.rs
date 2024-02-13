use serde_derive::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct SystemInfo {
    pub gpu: String,
    pub ram: u64, // RAM in MB
    pub internet_speed: String,
    pub token: String,
    pub unique_id: String,
    pub input: i32,
    pub output: i32
}
