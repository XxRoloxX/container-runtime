use container_runtime::common::socket::get_daemon_socket_stream;
use dotenv::dotenv;
use log::{error, info};

pub mod cli;
fn main() {
    dotenv().ok();
    env_logger::init();
    match cli::run_cli(get_daemon_socket_stream()) {
        Ok(_) => info!("Command sent successfully"),
        Err(e) => error!("Error: {}", e),
    };
}
