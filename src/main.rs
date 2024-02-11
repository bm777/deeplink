mod db;
mod http;
mod utils;
mod models;
mod constants;
mod controller;

use std::net::TcpListener;
use crate::db::{init_database};
use crate::http::{handle_request};


fn main() {

    // set up the database
    if let Err(e) = init_database() {
        println!("Failed to initialize database: {}", e);
        return;
    }

    // start the server and print port
    let listener = TcpListener::bind(format!("0.0.0.0:11424")).unwrap();
    println!("Server listening on port 11424");

    // handle incoming requests
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_request(stream);
            }
            Err(e) => println!("Failed to establish a connection: {}", e),
        }
    }

}