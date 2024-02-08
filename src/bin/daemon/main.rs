use dotenv::dotenv;

pub mod container;
pub mod container_runner;
pub mod controller;
pub mod deployment;
pub mod image;
pub mod server;

fn main() {
    println!("Running server");
    dotenv().ok();
    match server::run_server() {
        Ok(_) => {
            println!("All good")
        }
        Err(err) => {
            println!("Shit {}", err)
        }
    };
}
