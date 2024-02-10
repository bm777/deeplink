
use serde_json;
use crate::models::Hardware;
use rusqlite::{Connection, Result};
use crate::constants::{INTERNAL_ERROR};
use dotenv::dotenv;
use std::env;

// utilities functions
// extract the id from the request
pub fn get_id(request: &str) -> &str {
    request.split("/").nth(2).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}

// deserialize hardware from request body without id
pub fn get_hardware_request_body(request: &str) -> Result<Hardware, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}

// establish connection to the database
pub fn establish_connection() -> Result<Connection, (String, String)> {
    dotenv().ok();
    let database_path = env::var("DATABASE_PATH").expect("DATABASE_PATH must be set");
    
    match Connection::open(&database_path) {
        Ok(conn) => Ok(conn),
        Err(e) => Err((INTERNAL_ERROR.to_string(), format!("Failed to open connection: {}", e)))
    }
}