use clap::Parser;
use container_runtime::common::{
    commands::ContainerCommand,
    socket::{get_client_socket_listener, ConnectionHandler, SocketListener, SocketStream},
};
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

    let mut socket_listener = get_client_socket_listener();
    socket_listener.prepare_socket()?;
    stream.send_command(args.command.as_ref().unwrap())?;
    // wait_for_program_output(socket_listener)?;
    Ok(())
}

// fn log_data(data: Vec<u8>) {
//     println!("Got data: {:?}", String::from_utf8(data));
// }

fn wait_for_program_output(mut socket_listener: Box<dyn SocketListener>) -> Result<(), String> {
    let mut handler: ConnectionHandler = Box::from(move |data| log_data(data));
    socket_listener.listen(&mut handler)?;
    Ok(())
}
