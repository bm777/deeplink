use crate::models::{Hardware, Credentials};
use crate::constants::{OK_RESPONSE, NOT_FOUND, INTERNAL_ERROR};
use crate::utils::{get_id, get_hardware_request_body, get_credentials_request_body};
use crate::utils::establish_connection;


// CRUD functinons
// Create resource
pub fn create_resource(resource_type: &str, request: &str) -> (String, String) {
    match (resource_type, request) {
        ("hardware", _) => {
            
            match (
                get_hardware_request_body(&request), 
                establish_connection()
            ) {
                (Ok(hardware), Ok(conn)) => {
                    println!("Creating hardware");
                    conn.execute(
                        "INSERT INTO hardware (gpu, ram, internet_speed) VALUES (?1, ?2, ?3)",
                        &[&hardware.gpu, &hardware.ram, &hardware.internet_speed]
                    ).expect("Failed to insert hardware");
                    println!("Hardware created");

                    (OK_RESPONSE.to_string(), "hardware created".to_string())
                },
                _ => (INTERNAL_ERROR.to_string(), "error creating hardware".to_string())
            }
        },
        ("credentials", _) => {
            match (
                get_credentials_request_body(request), 
                establish_connection()
            ) {
                (Ok(credentials), Ok(conn)) => {
                    conn.execute(
                        "INSERT INTO credentials (token, uniq_id) VALUES (?1, ?2)",
                        &[&credentials.token, &credentials.uniq_id]
                    ).expect("Failed to insert credentials");

                    (OK_RESPONSE.to_string(), "credentials created".to_string())
                },
                _ => (INTERNAL_ERROR.to_string(), "error creating credentials".to_string())
            }
        },
        _ => (NOT_FOUND.to_string(), "invalid resource type".to_string())
    }
}
// Read resource
pub fn read_resource(resource_type: &str, request: &str) -> (String, String) {
    match (resource_type, request) {
        ("hardware", _) => {
            match (
                get_id(&request).parse::<i32>(), 
                establish_connection()
            ) {
                (Ok(id), Ok(conn)) => {
                    println!("Reading hardware");
                    let mut statement = conn.prepare("SELECT * FROM hardware WHERE id = ?1").expect("Failed to prepare statement");
                    let hardware_iter = statement.query_map(&[&id], |row| {
                        Ok(Hardware {
                            id: row.get(0)?,
                            gpu: row.get(1)?,
                            ram: row.get(2)?,
                            internet_speed: row.get(3)?
                        })
                    }).expect("Failed to query hardware");
    
                    let mut hardwares = Vec::new();
                    for hardware in hardware_iter {
                        hardwares.push(hardware.unwrap());
                    }
                    println!("Hardware read");
    
                    (OK_RESPONSE.to_string(), serde_json::to_string(&hardwares).unwrap())
                },
                _ => (INTERNAL_ERROR.to_string(), "error reading hardware".to_string())
            }
        },
        ("credentials", _) => {
            match (
                get_id(&request).parse::<i32>(), 
                establish_connection()
            ) {
                (Ok(id), Ok(conn)) => {
                    let mut statement = conn.prepare("SELECT * FROM credentials WHERE id = ?1").expect("Failed to prepare statement");
                    let credentials_iter = statement.query_map(&[&id], |row| {
                        Ok(Credentials {
                            id: row.get(0)?,
                            token: row.get(1)?,
                            uniq_id: row.get(2)?
                        })
                    }).expect("Failed to query credentials");
    
                    let mut credentials = Vec::new();
                    for credential in credentials_iter {
                        credentials.push(credential.unwrap());
                    }
    
                    (OK_RESPONSE.to_string(), serde_json::to_string(&credentials).unwrap())
                },
                _ => (INTERNAL_ERROR.to_string(), "error reading credentials".to_string())
            }
        },
        _ => (NOT_FOUND.to_string(), "invalid resource type".to_string())
    }
}
// Read all resources (hardware, credentials)
pub fn read_all_resources(resource_type: &str, _request: &str) -> (String, String) {
   match (resource_type, _request) {
        ("hardware", _) => {
            match establish_connection() {
                Ok(conn) => {
                    println!("Reading all hardware");
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
                    println!("All hardware read");
    
                    (OK_RESPONSE.to_string(), serde_json::to_string(&hardwares).unwrap())
                },
                _ => (INTERNAL_ERROR.to_string(), "error reading hardware".to_string())
            }
        },
        ("credentials", _) => {
            match establish_connection() {
                Ok(conn) => {
                    let mut statement = conn.prepare("SELECT * FROM credentials").expect("Failed to prepare statement");
                    let credentials_iter = statement.query_map([], |row| {
                        Ok(Credentials {
                            id: row.get(0)?,
                            token: row.get(1)?,
                            uniq_id: row.get(2)?
                        })
                    }).expect("Failed to query credentials");
    
                    // let mut credentials = Vec::new();
                    let mut credentials: Vec<Credentials> = Vec::new();
                    for credential in credentials_iter {
                        credentials.push(credential.unwrap());
                    }
    
                    (OK_RESPONSE.to_string(), serde_json::to_string(&credentials).unwrap())
                },
                _ => (INTERNAL_ERROR.to_string(), "error reading credentials".to_string())
            }
        },
        _ => (NOT_FOUND.to_string(), "invalid resource type".to_string())
    }
}
// Update resource
pub fn update_resource(resource_type: &str, request: &str) -> (String, String) {
    match (resource_type, request) {
        ("hardware", _) => {
            match (
                get_id(&request).parse::<i32>(), 
                get_hardware_request_body(&request), 
                establish_connection()
            ) {
                (Ok(id), Ok(hardware), Ok(conn)) => {
                    println!("Updating hardware");
                    conn.execute(
                        "UPDATE hardware SET gpu = ?1, ram = ?2, internet_speed = ?3 WHERE id = ?4",
                        &[&hardware.gpu, &hardware.ram, &hardware.internet_speed, &id.to_string()]
                    ).unwrap();
                    println!("Hardware updated");

                    (OK_RESPONSE.to_string(), "hardware updated".to_string())
                },
                _ => (INTERNAL_ERROR.to_string(), "Error updating hardware".to_string())
            }
        },
        ("credentials", _) => {
            match (
                get_id(&request).parse::<i32>(), 
                get_credentials_request_body(request), 
                establish_connection()
            ) {
                (Ok(id), Ok(credentials), Ok(conn)) => {
                    conn.execute(
                        "UPDATE credentials SET token = ?1, uniq_id = ?2 WHERE id = ?3",
                        &[&credentials.token, &credentials.uniq_id, &id.to_string()]
                    ).unwrap();

                    (OK_RESPONSE.to_string(), "credentials updated".to_string())
                }
                _ => (INTERNAL_ERROR.to_string(), "Error updating credentials".to_string())
            }
        },
        _ => (NOT_FOUND.to_string(), "invalid resource type".to_string())
    }
}
// Delete resource
pub fn delete_resource(resource_type: &str, request: &str) -> (String, String) {
    match (resource_type, request) {
        ("hardware", _) => {
            match (
                get_id(&request).parse::<i32>(), 
                establish_connection()
            ) {
                (Ok(id), Ok(conn)) => {
                    println!("Deleting hardware");
                    conn.execute(
                        "DELETE FROM hardware WHERE id = ?1",
                        &[&id]
                    ).unwrap();
                    println!("Hardware deleted");

                    (OK_RESPONSE.to_string(), "hardware deleted".to_string())
                },
                _ => (INTERNAL_ERROR.to_string(), "error deleting hardware".to_string())
            }
        },
        ("credentials", _) => {
            match (
                get_id(&request).parse::<i32>(), 
                establish_connection()
            ) {
                (Ok(id), Ok(conn)) => {
                    conn.execute(
                        "DELETE FROM credentials WHERE id = ?1",
                        &[&id]
                    ).unwrap();

                    (OK_RESPONSE.to_string(), "credentials deleted".to_string())
                },
                _ => (INTERNAL_ERROR.to_string(), "error deleting credentials".to_string())
            }
        },
        _ => (NOT_FOUND.to_string(), "invalid resource type".to_string())
    }
}