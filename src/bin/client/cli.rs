use clap::Parser;
use container_runtime::common::{commands::ContainerCommand, socket::SocketStream};
use dotenv::dotenv;
use log::{error, info};

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
        Some(ContainerCommand::Start {
            image,
            container_id,
            command,
            args,
        }) => {
            info!(
                "Starting container: {}, {}, {}, {}",
                container_id,
                image,
                command,
                args.join(" ")
            );
        }
        Some(ContainerCommand::Stop { container_id }) => {
            info!("Stopping container: {}", container_id);
        }
        Some(ContainerCommand::Create {
            container_id,
            image,
        }) => {
            info!("Creating container: {} with image: {}", container_id, image);
        }
        Some(ContainerCommand::Build {
            image_id,
            dockerfile,
        }) => {
            info!(
                "Building image: {} with Dockerfile: {}",
                image_id, dockerfile
            );
        }
        None => {
            error!("No command provided");
        }
    }
    stream.send_command(args.command.as_ref().unwrap())?;
    Ok(())
}
