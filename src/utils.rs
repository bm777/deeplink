// use std::process::Command;
use hyper::{Response, Request, StatusCode};
use hyper::body::Body;
use hyper::header::CONTENT_TYPE;
use crate::db::{load_system_info};
use std::convert::Infallible;
use serde_urlencoded::from_str as deserialize_qs;
use std::collections::HashMap;

pub async fn check_hardware() -> Result<Response<Body>, Infallible> {
    let info = load_system_info().expect("Failed to load system info");
    let threshold = 1024 * 1024 * 8;

    let body = if info.ram >= threshold {
        serde_json::to_string(&serde_json::json!({ "status": "success", "data": info })).expect("Failed to serialize system info")
    } else {
        serde_json::to_string(&serde_json::json!({ "status": "error", "message": "Insufficient RAM" })).expect("Failed to serialize error message")
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(body))
        .expect("Failed to construct response"))
}

pub async fn download_binary(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // Extract query parameters
    let query = req.uri().query().unwrap_or_default();
    let query_params: HashMap<String, String> = deserialize_qs(query).unwrap_or_default();

    if let Some(model) = query_params.get("model") {
        println!("Downloading model: {}", model);
        let response_body = serde_json::to_string(&serde_json::json!({ "status": "success", "message": "Model downloaded" })).expect("Failed to serialize response");
        Ok(Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(response_body))
            .expect("Failed to construct response"))
    } else {
        // If the `model` query parameter is missing, return an error response
        Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("Missing model query parameter"))
            .expect("Failed to construct response"))
    }
}

pub async fn start_inference() -> Result<Response<Body>, Infallible> {
    // inference code
    Ok(Response::new(Body::from("Inference started")))
}