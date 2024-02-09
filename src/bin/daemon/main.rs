use container_runtime::common::socket::get_socket_listener;
use dotenv::dotenv;

pub mod container;
pub mod controller;
pub mod deployment;
pub mod image;
pub mod runner;
pub mod server;

fn main() {
    println!("Running server");
    dotenv().ok();
    match server::run_server(get_socket_listener()) {
        Ok(_) => {
            println!("All good")
        }
        Err(err) => {
            println!("Shit {}", err)
        }
    };
}
