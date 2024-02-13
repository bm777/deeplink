use anyhow::Result;
use std::fs;
use std::path::Path;
use crate::models::SystemInfo;


const DB_PATH: &str = "system.toml";

pub fn load_system_info() -> Result<SystemInfo> {
    let db_path = Path::new(DB_PATH);
    if db_path.exists() {
        let contents = fs::read_to_string(db_path)?;
        let info: SystemInfo = toml::from_str(&contents)?;
        Ok(info)
    } else {
        Ok(SystemInfo {
            gpu: String::new(),
            ram: 0,
            internet_speed: String::new(),
            token: String::new(),
            unique_id: String::new(),
            input: 0,
            output: 0
        })
    }
}

pub fn save_system_info(info: &SystemInfo) -> Result<()> {
    let contents = toml::to_string(&info)?;
    fs::write(DB_PATH, contents)?;
    Ok(())
}
