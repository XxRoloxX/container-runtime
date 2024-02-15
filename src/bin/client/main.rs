use container_runtime::common::sockets::get_container_command_stream;
use log::{error, info};

pub mod cli;
fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    match cli::run_cli(get_container_command_stream()) {
        Ok(_) => info!("Command sent successfully"),
        Err(e) => error!("Error: {}", e),
    };
}
