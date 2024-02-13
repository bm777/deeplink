mod models;
mod gpu;
mod db;
mod system_info;

use crate::db::{load_system_info, save_system_info};
use crate::system_info::get_system_info;
use std::net::TcpListener;

fn main() {

    match load_system_info() {
        Ok(info) if info.ram >0 => {
            let new_info = get_system_info();
            println!("System info: {:?}", new_info);
            save_system_info(&new_info).expect("Failed to save system info");
        },
        _ => {
            // time to save system infor if not already saved
            let info = get_system_info();
            println!("System info: {:?}", info);
            
            save_system_info(&info).expect("Failed to save system info");
        }

    }

    let listener = TcpListener::bind(format!("0.0.0.0:11424")).expect("Failed to bind to port");
    println!("Server listening on port 11424");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
            }
            Err(e) => println!("Failed to establish a connection: {}", e),
        }
    }

}