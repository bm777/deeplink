mod db;
mod gpu;
mod models;
mod system_info;
mod utils;

use crate::db::{load_system_info, save_system_info};
use crate::utils::{check_hardware, download_binary, start_inference};
use crate::system_info::get_system_info;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, StatusCode};
use hyper::header::CONTENT_TYPE;
use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::server::conn::AddrStream;

#[tokio::main]
async fn main() {

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

    let addr: SocketAddr = ([127, 0, 0, 1], 11424).into();

    let make_svc = make_service_fn(|_conn: &AddrStream| async {
        Ok::<_, Infallible>(service_fn(|req: Request<Body>| async move {

            match (req.method(), req.uri().path()) {
                (&hyper::Method::GET, "/") => Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header(CONTENT_TYPE, "application/json")
                    .body(Body::from(serde_json::json!({"data": "Welcome to deeplink static page!"}).to_string()))
                    .expect("Failed to construct response")),
                    
                (&hyper::Method::GET, "/check") => check_hardware().await,
                
                (&hyper::Method::GET, "/download") => download_binary(req).await,

                (&hyper::Method::GET, "/inference") => start_inference().await,

                _ => Ok(Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::from("Not Found"))
                    .expect("Failed to construct response")),
            }
        }))
    });

    let server = hyper::Server::bind(&addr).serve(make_svc);
    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}