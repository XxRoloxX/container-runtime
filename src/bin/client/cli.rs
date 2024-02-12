use clap::Parser;
use container_runtime::common::{
    commands::ContainerCommand,
    socket::{get_client_socket_listener, SocketStream},
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
    info!("Connected to the deamon");

    args.command
        .as_ref()
        .ok_or("No command provided".to_string())?;

    info!("{}", args.command.as_ref().unwrap());

    let mut socket_listener = get_client_socket_listener();
    socket_listener.prepare_socket()?;
    stream.send_command(args.command.as_ref().unwrap())?;
    Ok(())
}
