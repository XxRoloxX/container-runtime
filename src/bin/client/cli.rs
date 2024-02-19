use clap::Parser;
use container_runtime::common::{
    client_request::{ClientId, ClientRequest},
    commands::{feedback_commands::FeedbackCommand, runtime_commands::ContainerCommand},
    sockets::{
        generic_sockets_with_parsers::CommandHandler, get_client_socket_listener,
        sockets_with_parsers::container_commands_socket::ContainerCommandStream,
    },
};
use log::info;

use crate::router::route_feedback_command;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Option<ContainerCommand>,
}

pub fn run_cli(mut stream: ContainerCommandStream) -> Result<(), String> {
    let args = Cli::parse();
    stream.connect()?;
    info!("Connected to the deamon");

    let mut command = args.command.ok_or("No command provided".to_string())?;

    info!("{}", command);

    command.canonize_paths();

    let client_request = ClientRequest::new(command);

    stream.send_command(client_request.clone())?;

    listen_for_daemon_response(client_request.client_id)?;

    Ok(())
}

pub fn listen_for_daemon_response(client_id: ClientId) -> Result<(), String> {
    let mut socket_listener = get_client_socket_listener(client_id);
    socket_listener.prepare_socket()?;

    let handle_connection: CommandHandler<FeedbackCommand> = Box::from(route_feedback_command);

    socket_listener.listen(handle_connection)?;
    Ok(())
}
