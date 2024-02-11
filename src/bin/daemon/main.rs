#![feature(unix_sigpipe)]

use container_runtime::common::socket::get_daemon_socket_listener;
use dotenv::dotenv;
use log::{error, info};

pub mod container;
pub mod controllers;
pub mod deployment;
pub mod image_builder;
pub mod registry;
pub mod router;
pub mod runner;
pub mod server;

// #[unix_sigpipe = "sig_dfl"]
fn main() {
    println!("Running server");
    dotenv().ok();
    env_logger::init();
    match server::run_server(get_daemon_socket_listener()) {
        Ok(_) => {
            info!("Server was started");
        }
        Err(err) => {
            error!("Couldn't start server: {}", err);
        }
    };
}
