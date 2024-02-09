use clap::Parser;
use container_runtime::common::{commands::ContainerCommand, socket::SocketStream};
use dotenv::dotenv;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Option<ContainerCommand>,
}

pub fn run_cli(mut stream: Box<dyn SocketStream>) -> Result<(), String> {
    dotenv().ok();
    let args = Cli::parse();
    stream.connect()?;
    match &args.command {
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
    stream.send_command(args.command.as_ref().unwrap())?;
    Ok(())
}
