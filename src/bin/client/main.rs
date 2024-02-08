use dotenv::dotenv;

pub mod cli;
fn main() {
    dotenv().ok();
    match cli::run_cli() {
        Ok(_) => println!("Command sent successfully"),
        Err(e) => println!("Error: {}", e),
    };
}
