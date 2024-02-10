
use std::net::{TcpStream};
use std::io::{Read, Write};
use crate::constants::{NOT_FOUND};
use crate::controllers::hardware_controller::{create_hardware, read_hardware, read_all_hardware, update_hardware, delete_hardware};

pub fn handle_hardware(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(&String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_, content) = match &*request {
                r if r.starts_with("POST /hardware") => create_hardware(&r),
                r if r.starts_with("GET /hardware/") => read_hardware(&r),
                r if r.starts_with("GET /hardware") => read_all_hardware(&r),
                r if r.starts_with("PUT /hardware/") => update_hardware(&r),
                r if r.starts_with("DELETE /hardware/") => delete_hardware(&r),
                _ => (NOT_FOUND.to_string(), "404 not found".to_string())
            };

            stream.write_all(format!("{}{}", status_, content).as_bytes()).unwrap();
        },
        Err(e) => println!("Failed to read from connection: {}", e),
    }
}