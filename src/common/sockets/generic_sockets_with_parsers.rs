use log::error;
use serde::{de::DeserializeOwned, Serialize};

use super::{CommandHandler, ConnectionHandler, SocketListener, SocketStream};

pub struct GenericCommandStream(Box<dyn SocketStream>);
pub struct GenericCommandListener(Box<dyn SocketListener>);

pub trait SocketListenerWithParser<T>
where
    T: DeserializeOwned,
{
    fn prepare_socket(&mut self) -> Result<(), String>;
    fn listen(&mut self, handle_connection: CommandHandler<T>) -> Result<(), String>;
}

pub trait SocketStreamWithParser<T> {
    fn connect(&mut self) -> Result<i32, String>;
    fn send_command(&mut self, command: T) -> Result<(), String>
    where
        T: Serialize;
}

impl GenericCommandStream {
    pub fn new(socket: Box<dyn SocketStream>) -> GenericCommandStream {
        GenericCommandStream(socket)
    }
}

impl<T: Serialize> SocketStreamWithParser<T> for GenericCommandStream {
    fn connect(&mut self) -> Result<i32, String> {
        self.0.connect()
    }

    fn send_command(&mut self, command: T) -> Result<(), String>
    where
        T: Serialize,
    {
        let message = serde_json::to_string(&command)
            .map_err(|e| format!("Couldn't serialize command {}", e))?;

        self.0.send_command(&message.as_bytes().to_vec())?;
        Ok(())
    }
}

impl GenericCommandListener {
    pub fn new(socket: Box<dyn SocketListener>) -> GenericCommandListener {
        GenericCommandListener(socket)
    }
}

impl<T: DeserializeOwned + 'static> SocketListenerWithParser<T> for GenericCommandListener {
    fn prepare_socket(&mut self) -> Result<(), String> {
        self.0.prepare_socket()
    }

    fn listen(&mut self, mut handle_connection: CommandHandler<T>) -> Result<(), String> {
        let mut handler: ConnectionHandler = Box::from(move |data: Vec<u8>| {
            let command: Result<T, String> = parse_command(data);
            match command {
                Ok(command) => handle_connection(command),
                Err(err) => error!("{}", err),
            }
        });

        self.0.listen(&mut handler)?;
        Ok(())
    }
}

pub fn parse_command<T: DeserializeOwned>(buf: Vec<u8>) -> Result<T, String> {
    let stringified_data = String::from_utf8(buf.clone())
        .map_err(|e| format!("Couldn't parse command to string {}", e))?;

    let command: T = serde_json::from_str(&stringified_data).map_err(|e| {
        format!(
            "Couldn't parse the command json: {}: {}",
            stringified_data, e
        )
    })?;
    Ok(command)
}
