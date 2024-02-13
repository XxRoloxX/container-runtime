use clap::Parser;
use container_runtime::common::{
    commands::ContainerCommand, sockets::container_commands_socket::ContainerCommandStream,
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

    // let mut socket_listener = get_client_socket_listener();
    // socket_listener.prepare_socket()?;
    stream.send_command(args.command.unwrap())?;

    Ok(())
}
//
// pub fn wait_for_unix_socket_message() -> Result<(), String> {
//     let mut socket_listener = get_client_socket_listener();
//     socket_listener.prepare_socket()?;
//
//     let mut handle_connection: ConnectionHandler = Box::from(|buffer: Vec<u8>| {
//         let message = String::from_utf8(buffer).unwrap();
//         info!("Received message: {}", message);
//     });
//
//     socket_listener.listen(&mut handle_connection)?;
//     Ok(())
// }
