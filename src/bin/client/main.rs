use container_runtime::common::socket::get_socket_stream;
use dotenv::dotenv;

pub mod cli;
fn main() {
    dotenv().ok();
    match cli::run_cli(get_socket_stream()) {
        Ok(_) => println!("Command sent successfully"),
        Err(e) => println!("Error: {}", e),
    };
}
