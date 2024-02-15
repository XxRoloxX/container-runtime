use container_runtime::common::{logs::configure_logging, sockets::get_container_command_listener};
use log::{error, info};

pub mod container;
pub mod controllers;
pub mod image_builder;
pub mod router;
pub mod runner;
pub mod server;

fn main() {
    configure_logging().expect("Failed to get INSTALL_PATH");
    match server::run_server(get_container_command_listener()) {
        Ok(_) => {
            info!("Server was started");
        }
        Err(err) => {
            error!("Couldn't start server: {}", err);
        }
    };
}
