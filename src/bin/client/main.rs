use dotenv::dotenv;

pub mod cli;
fn main() {
    dotenv().ok();
    cli::run_cli();
}
