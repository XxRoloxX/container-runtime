use clap::Parser;
use container_runtime::common::{
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

    args.command
        .as_ref()
        .ok_or("No command provided".to_string())?;

    info!("{}", args.command.as_ref().unwrap());

    stream.send_command(args.command.unwrap())?;

    listen_for_daemon_response()?;

    Ok(())
}

pub fn listen_for_daemon_response() -> Result<(), String> {
    let mut socket_listener = get_client_socket_listener();
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
                    // return Err(format!("Container {} exited", name).to_string());
                    info!("Container {} exited", name);
                    Ok(ConnectionStatus::Finished)
                }
                FeedbackCommand::ImageBuilt { image } => {
                    // return Err(format!("Image {} built", image.id).to_string());
                    info!("Image {} built", image.id);
                    Ok(ConnectionStatus::Finished)
                }
            }
        });

    socket_listener.listen(handle_connection)?;
    Ok(())
}
