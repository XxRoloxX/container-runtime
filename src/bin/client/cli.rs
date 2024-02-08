use std::{io::Write, os::unix::net::UnixStream};

use clap::{error::Result, Parser};
use container_runtime::common::{commands::ContainerCommand, socket::SOCKET_PATH};
use dotenv::dotenv;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Option<ContainerCommand>,
}

struct UnixStreamWrapper {
    path: String,
    socket: Option<UnixStream>,
}

impl UnixStreamWrapper {
    pub fn new() -> Self {
        UnixStreamWrapper {
            path: SOCKET_PATH.to_string(),
            socket: None,
        }
    }

    pub fn connect(&mut self) -> Result<(), String> {
        let socket = UnixStream::connect(self.path.to_string())
            .map_err(|e| format!("Failed to connect to socket {}", e))?;

        self.socket = Some(socket);
        Ok(())
    }
    pub fn send_command(&mut self, command: &ContainerCommand) -> Result<(), String> {
        match &mut self.socket {
            Some(socket) => {
                let message = serde_json::to_string(&command)
                    .map_err(|e| format!("Couldn't serialize command {}", e))?;
                socket
                    .write(message.as_bytes())
                    .map_err(|e| format!("Couldn't send a command: {}", e))?;
                Ok(())
            }
            None => return Err("Not connected to socket!".to_string()),
        }
    }
}

pub fn run_cli() -> Result<(), String> {
    dotenv().ok();
    let args = Cli::parse();
    let mut stream = UnixStreamWrapper::new();
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

pub fn main() {
    run_cli();
}
