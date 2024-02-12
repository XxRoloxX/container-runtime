use super::{
    CommandHandler, ConnectionHandler, SocketListener, SocketListenerWithParser, SocketStream,
    SocketStreamWithParser,
};
use crate::common::commands::ContainerCommand;
use log::error;
pub struct ContainerCommandStream(Box<dyn SocketStream>);
pub struct ContainerCommandListener(Box<dyn SocketListener>);

// pub type ContainerCommandHandler = Box<dyn FnMut(ContainerCommand)>;

impl ContainerCommandStream {
    pub fn new(socket: Box<dyn SocketStream>) -> ContainerCommandStream {
        ContainerCommandStream(socket)
    }
}

impl SocketStreamWithParser<ContainerCommand> for ContainerCommandStream {
    fn connect(&mut self) -> Result<i32, String> {
        self.0.connect()
    }

    fn send_command(&mut self, command: &ContainerCommand) -> Result<(), String> {
        let message = serde_json::to_string(&command)
            .map_err(|e| format!("Couldn't serialize command {}", e))?;

        self.0.send_command(&message.as_bytes().to_vec())?;
        Ok(())
    }
}

impl ContainerCommandListener {
    pub fn new(socket: Box<dyn SocketListener>) -> ContainerCommandListener {
        ContainerCommandListener(socket)
    }
}

impl SocketListenerWithParser<ContainerCommand> for ContainerCommandListener {
    fn prepare_socket(&mut self) -> Result<(), String> {
        self.0.prepare_socket()
    }

    fn listen(
        &mut self,
        mut handle_connection: CommandHandler<ContainerCommand>,
    ) -> Result<(), String> {
        let mut handler: ConnectionHandler = Box::from(move |data: Vec<u8>| {
            let command = parse_command(&data);
            match command {
                Ok(command) => handle_connection(command),
                Err(err) => error!("{}", err),
            }
        });

        self.0.listen(&mut handler)?;
        Ok(())
    }
}

pub fn parse_command(buf: &Vec<u8>) -> Result<ContainerCommand, String> {
    let stringified_data = String::from_utf8(buf.clone())
        .map_err(|e| format!("Couldn't parse command to string {}", e))?;

    let command: ContainerCommand =
        serde_json::from_str(stringified_data.as_ref()).map_err(|e| {
            format!(
                "Couldn't parse the command json: {}: {}",
                stringified_data, e
            )
        })?;
    Ok(command)
}
