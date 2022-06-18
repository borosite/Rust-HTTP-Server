#![allow(dead_code)]    //silence the compiler warnings

use server::HttpServer;
use website_handler::WebsiteHandler;
use std::env;

mod server;
mod http;
mod website_handler;

fn main() {
    let default_path = format!("{}\\public", env!("CARGO_MANIFEST_DIR"));    //latter gives the root directory path
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let server = HttpServer::new("127.0.0.1:8080".to_string());
    println!("public path: {}", public_path);
    server.run(WebsiteHandler::new(public_path));
}