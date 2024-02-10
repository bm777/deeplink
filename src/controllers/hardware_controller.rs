use crate::models::Hardware;
use crate::constants::{OK_RESPONSE, NOT_FOUND, INTERNAL_ERROR};
use crate::utils::{get_id, get_hardware_request_body};
use crate::utils::establish_connection;


// CRUD functinons
// Create hardware
pub fn create_hardware(request: &str) -> (String, String) {
    match (
        get_hardware_request_body(&request), 
        establish_connection()
    ) {
        (Ok(hardware), Ok(conn)) => {
            conn.execute(
                "INSERT INTO hardware (gpu, ram, internet_speed) VALUES (?1, ?2, ?3)",
                &[&hardware.gpu, &hardware.ram, &hardware.internet_speed]
            ).unwrap();

            (OK_RESPONSE.to_string(), "Hardware created".to_string())
        },
        _ => (INTERNAL_ERROR.to_string(), "Error creating hardware".to_string())
    }
}
// Read hardware
pub fn read_hardware(request: &str) -> (String, String) {
    match (
        get_id(&request).parse::<i32>(), 
        establish_connection()
    ) {
        (Ok(id), Ok(conn)) => {
            let mut statement = conn.prepare("SELECT * FROM hardware WHERE id = ?1").expect("Failed to prepare statement");
            let mut hardware_iter = statement.query_map(&[&id], |row| {
                Ok(Hardware {
                    id: row.get(0)?,
                    gpu: row.get(1)?,
                    ram: row.get(2)?,
                    internet_speed: row.get(3)?
                })
            }).expect("Failed to query hardware");

            let hardware = match hardware_iter.next() {
                Some(hardware) => hardware.unwrap(),
                None => return (NOT_FOUND.to_string(), "Hardware not found".to_string())
            };

            (OK_RESPONSE.to_string(), serde_json::to_string(&hardware).unwrap())
        },
        _ => (INTERNAL_ERROR.to_string(), "Error reading hardware".to_string())
    }
}
// Read all hardware
pub fn read_all_hardware(_request: &str) -> (String, String) {
    match establish_connection() {
        Ok(conn) => {
            let mut statement = conn.prepare("SELECT * FROM hardware").expect("Failed to prepare statement");
            let hardware_iter = statement.query_map([], |row| {
                Ok(Hardware {
                    id: row.get(0)?,
                    gpu: row.get(1)?,
                    ram: row.get(2)?,
                    internet_speed: row.get(3)?
                })
            }).expect("Failed to query hardware");

            // let mut hardwares = Vec::new();
            let mut hardwares: Vec<Hardware> = Vec::new();
            for hardware in hardware_iter {
                hardwares.push(hardware.unwrap());
            }

            (OK_RESPONSE.to_string(), serde_json::to_string(&hardwares).unwrap())
        },
        _ => (INTERNAL_ERROR.to_string(), "Error reading hardware".to_string())
    }
}
// Update hardware
pub fn update_hardware(request: &str) -> (String, String) {
    match (
        get_id(&request).parse::<i32>(), 
        get_hardware_request_body(&request), 
        establish_connection()
    ) {
        (Ok(id), Ok(hardware), Ok(conn)) => {
            conn.execute(
                "UPDATE hardware SET gpu = ?1, ram = ?2, internet_speed = ?3 WHERE id = ?4",
                &[&hardware.gpu, &hardware.ram, &hardware.internet_speed, &id.to_string()]
            ).unwrap();

            (OK_RESPONSE.to_string(), "Hardware updated".to_string())
        }
        _ => (INTERNAL_ERROR.to_string(), "Error updating hardware".to_string())
    }
}
// Delete hardware
pub fn delete_hardware(request: &str) -> (String, String) {
    match (
        get_id(&request).parse::<i32>(), 
        establish_connection()
    ) {
        (Ok(id), Ok(conn)) => {
            let rows = conn
                .execute("DELETE FROM hardware WHERE id = ?1", &[&id])
                .unwrap();

            if rows == 0 {
                return (NOT_FOUND.to_string(), "Hardware not found".to_string());
            }

            (OK_RESPONSE.to_string(), "Hardware deleted".to_string())
        },
        _ => (INTERNAL_ERROR.to_string(), "Error deleting hardware".to_string())
    }
}

