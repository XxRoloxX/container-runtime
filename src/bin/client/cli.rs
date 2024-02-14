use clap::Parser;
use container_runtime::common::{
    feedback_commands::FeedbackCommand,
    runtime_commands::ContainerCommand,
    sockets::{
        container_commands_socket::ContainerCommandStream,
        generic_sockets_with_parsers::CommandHandler, get_client_socket_listener,
    },
};
use dotenv::dotenv;
use log::info;

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
    wait_for_unix_socket_message()?;

    Ok(())
}

pub fn wait_for_unix_socket_message() -> Result<(), String> {
    let mut socket_listener = get_client_socket_listener();
    socket_listener.prepare_socket()?;

    let handle_connection: CommandHandler<FeedbackCommand> =
        Box::from(|command: FeedbackCommand| {
            info!("Received message: {}", command);
        });

    socket_listener.listen(handle_connection)?;
    Ok(())
}
