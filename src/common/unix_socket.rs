use super::commands::ContainerCommand;
use super::socket::{SocketListener, SocketStream};
use clap::error::Result;
use nix::unistd::unlink;
use std::io::{Read, Write};
use std::os::unix::net::UnixListener;
use std::os::unix::net::UnixStream;

pub static SOCKET_PATH: &'static str = "/tmp/rust.sock";

pub struct UnixSocketListener {
    addr: String,
    listener: Option<UnixListener>,
}

impl UnixSocketListener {
    pub fn new() -> Self {
        UnixSocketListener {
            addr: SOCKET_PATH.to_string(),
            listener: None,
        }
    }

    fn get_listener(&mut self) -> Result<&UnixListener, String> {
        let listener = self
            .listener
            .as_ref()
            .ok_or(format!("Unix socket is not prepared"))?;

        Ok(listener)
    }
}

impl SocketListener for UnixSocketListener {
    fn prepare_socket(&mut self) -> Result<(), String> {
        unlink(self.addr.as_str()).unwrap_or_default();

        let listener = UnixListener::bind(SOCKET_PATH)
            .map_err(|e| format!("Failed to create listener: {}", e))?;

        self.listener = Some(listener);

        Ok(())
    }
    fn listen(&mut self, handle_connection: Box<dyn Fn(Vec<u8>)>) -> Result<(), String> {
        let listener = self.get_listener()?;
        for stream in listener.incoming() {
            let mut connection = stream.map_err(|e| format!("Connection faield {}", e))?;
            println!("Got connection");
            let mut buf: [u8; 100] = [0u8; 100];

            let read_bytes = connection
                .read(&mut buf)
                .map_err(|e| format!("Failed to read data: {}", e))?;

            let trimmed_buffer = buf[0..read_bytes].to_vec();

            handle_connection(trimmed_buffer);
        }

        Ok(())
    }
}

pub struct UnixSocketStream {
    path: String,
    socket: Option<UnixStream>,
}

impl UnixSocketStream {
    pub fn new() -> Self {
        UnixSocketStream {
            path: SOCKET_PATH.to_string(),
            socket: None,
        }
    }
}
impl SocketStream for UnixSocketStream {
    fn connect(&mut self) -> Result<(), String> {
        let socket = UnixStream::connect(self.path.to_string())
            .map_err(|e| format!("Failed to connect to socket {}", e))?;

        self.socket = Some(socket);
        Ok(())
    }
    fn send_command(&mut self, command: &ContainerCommand) -> Result<(), String> {
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
