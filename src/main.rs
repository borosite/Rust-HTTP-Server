#![allow(dead_code)]    //silence the compiler warnings

use server::HttpServer;
use website_handler::WebsiteHandler;

mod server;
mod http;
mod website_handler;

fn main() {
    let server = HttpServer::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler);
}