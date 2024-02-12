use std::net::TcpStream;
use std::io::{Read, Write};
use crate::constants::NOT_FOUND;
use crate::controller::controller::{create_resource, read_resource, read_all_resources, update_resource, delete_resource};

pub fn handle_request(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(&String::from_utf8_lossy(&buffer[..size]).as_ref());
            let resource_type = extract_resource_type(&request);
            if let Some(resource_type) = resource_type {
                let (status_, content) = match &*request {
                    r if r.starts_with(&format!("POST /{}", resource_type)) => create_resource(&resource_type, &r),
                    r if r.starts_with(&format!("GET /{}/", resource_type)) => read_resource(&resource_type, &r),
                    r if r.starts_with(&format!("GET /{}", resource_type)) => read_all_resources(&resource_type, &r),
                    r if r.starts_with(&format!("PUT /{}/", resource_type)) => update_resource(&resource_type, &r),
                    r if r.starts_with(&format!("DELETE /{}/", resource_type)) => delete_resource(&resource_type, &r),
                    _ => (NOT_FOUND.to_string(), "404 not found".to_string())
                };
                stream.write_all(format!("{}{}", status_, content).as_bytes()).unwrap();
            } else {
                stream.write_all(format!("{}{}", NOT_FOUND.to_string(), "404 not found".to_string()).as_bytes()).unwrap();
            }
        },
        Err(e) => println!("Failed to read from connection: {}", e),
    }
}

fn extract_resource_type(request: &str) -> Option<String> {
    let parts: Vec<&str> = request.split_whitespace().collect();
    let path = parts.get(1).unwrap_or(&"").to_string();
    // println!("Path: {}", path);

    if path.starts_with("/hardware") {
        Some("hardware".to_string())
    } else if path.starts_with("/credentials") {
        Some("credentials".to_string())
    } else {
        None
    }
}