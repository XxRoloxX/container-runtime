use clap::Parser;
use dotenv::dotenv;

use crate::common::commands::ContainerCommand;
#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Option<ContainerCommand>,
}

pub fn run_cli() {
    dotenv().ok();
    let args = Cli::parse();

    match args.command {
        Some(ContainerCommand::Start { container_id }) => {
            println!("Starting container: {}", container_id);
        }
        Some(ContainerCommand::Stop { container_id }) => {
            println!("Stopping container: {}", container_id);
        }
        Some(ContainerCommand::Create {
            container_id,
            image,
        }) => {
            println!("Creating container: {} with image: {}", container_id, image);
        }
        None => {
            println!("No command provided");
        }
    }
}
