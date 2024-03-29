use container_runtime::common::{logs::configure_logging, sockets::get_container_command_stream};
use log::{error, info};

pub mod cli;
pub mod controllers;
pub mod router;
fn main() {
    configure_logging().expect("Failed to get INSTALL_PATH");
    match cli::run_cli(get_container_command_stream()) {
        Ok(_) => info!("Exited successfully"),
        Err(e) => error!("Error: {}", e),
    };
}
