use clap::error::Result;
use log::info;
use nix::unistd::unlink;
use std::io::{Read, Write};
use std::os::fd::AsRawFd;
use std::os::unix::net::UnixListener;
use std::os::unix::net::UnixStream;

use crate::common::sockets::{
    ConnectionCommand, ConnectionHandler, ConnectionStatus, SocketListener, SocketStream,
};

pub struct UnixSocketListener {
    addr: String,
    listener: Option<UnixListener>,
}

impl UnixSocketListener {
    pub fn new(addr: &str) -> Self {
        UnixSocketListener {
            addr: addr.to_string(),
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

        let listener = UnixListener::bind(self.addr.as_str())
            .map_err(|e| format!("Failed to create listener: {}", e))?;

        self.listener = Some(listener);

        Ok(())
    }
    fn listen(&mut self, handle_connection: &mut ConnectionHandler) -> Result<(), String> {
        let listener = self.get_listener()?;

        for stream in listener.incoming() {
            let mut connection = stream.map_err(|e| format!("Connection faield {}", e))?;
            info!("Got connection");
            let mut buf: [u8; 200] = [0u8; 200];

            let read_bytes = connection
                .read(&mut buf)
                .map_err(|e| format!("Failed to read data: {}", e))?;

            let trimmed_buffer = buf[0..read_bytes].to_vec();

            match handle_connection(trimmed_buffer) {
                Ok(ConnectionStatus::Finished) => break,
                Ok(ConnectionStatus::Running) => continue,
                Err(e) => {
                    info!("Couldn't handle connection : {}", e);
                }
            }
        }

        Ok(())
    }
}

pub struct UnixSocketStream {
    addr: String,
    socket: Option<UnixStream>,
}

impl UnixSocketStream {
    pub fn new(addr: &str) -> Self {
        UnixSocketStream {
            addr: addr.to_string(),
            socket: None,
        }
    }
}

impl SocketStream for UnixSocketStream {
    fn connect(&mut self) -> Result<i32, String> {
        let socket = UnixStream::connect(self.addr.to_string())
            .map_err(|e| format!("Failed to connect to socket {}", e))?;

        let fd = socket.as_raw_fd();
        self.socket = Some(socket);
        Ok(fd)
    }
    fn send_command(&mut self, command: &ConnectionCommand) -> Result<(), String> {
        match &mut self.socket {
            Some(socket) => {
                socket
                    .write(command)
                    .map_err(|e| format!("Couldn't send a command: {}", e))?;
                Ok(())
            }
            None => return Err("Not connected to socket!".to_string()),
        }
    }
}
