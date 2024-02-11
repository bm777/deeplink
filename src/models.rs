use serde_derive::{Deserialize, Serialize};

// models
#[derive(Serialize, Deserialize)]
pub struct Hardware {
    pub id: Option<i32>,
    pub gpu: String,
    pub ram: String,
    pub internet_speed: String,
}

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub id: Option<i32>,
    pub token: String,
    pub uniq_id: String,
}