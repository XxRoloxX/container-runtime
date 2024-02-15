use clap::Parser;
use container_runtime::common::{
    client_request::{ClientId, ClientRequest},
    feedback_commands::FeedbackCommand,
    runtime_commands::ContainerCommand,
    sockets::{
        container_commands_socket::ContainerCommandStream,
        generic_sockets_with_parsers::CommandHandler, get_client_socket_listener, ConnectionStatus,
    },
    strace::run_strace,
};
use dotenv::dotenv;
use log::info;
use nix::unistd::Pid;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Option<ContainerCommand>,
}

pub fn run_cli(mut stream: ContainerCommandStream) -> Result<(), String> {
    dotenv().ok();
    let args = Cli::parse();
    stream.connect()?;
    info!("Connected to the deamon");

    let command = args.command.ok_or("No command provided".to_string())?;

    info!("{}", command);

    let client_request = ClientRequest::new(command);

    stream.send_command(client_request.clone())?;

    listen_for_daemon_response(client_request.client_id)?;

    Ok(())
}

pub fn listen_for_daemon_response(client_id: ClientId) -> Result<(), String> {
    let mut socket_listener = get_client_socket_listener(client_id);
    socket_listener.prepare_socket()?;

    let handle_connection: CommandHandler<FeedbackCommand> =
        Box::from(|command: FeedbackCommand| {
            info!("{}", command);
            match command {
                FeedbackCommand::ContainerStarted { pid, .. } => {
                    info!("Container started with pid {}", pid);
                    run_strace(Pid::from_raw(pid));
                    Ok(ConnectionStatus::Running)
                }
                FeedbackCommand::ContainerExited { name, .. } => {
                    info!("Container {} exited", name);
                    Ok(ConnectionStatus::Finished)
                }
                FeedbackCommand::ImageBuilt { image } => {
                    info!("Image {} built", image.id);
                    Ok(ConnectionStatus::Finished)
                }
            }
        });

    socket_listener.listen(handle_connection)?;
    Ok(())
}
