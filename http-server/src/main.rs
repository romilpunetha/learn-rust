#![allow(dead_code)]

extern crate core;

use std::env;

use http::{Method, Request};
use server::Server;

use crate::website_handler::WebsiteHandler;

mod server;
mod http;
mod website_handler;


fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}
