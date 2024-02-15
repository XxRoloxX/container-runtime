use self::{
    container_commands_socket::{ContainerCommandListener, ContainerCommandStream},
    feedback_commands_socket::{FeedbackCommandListener, FeedbackCommandStream},
    generic_sockets_with_parsers::{GenericCommandListener, GenericCommandStream},
    unix_socket::{UnixSocketListener, UnixSocketStream},
};

use super::feedback_commands::FeedbackCommand;

pub mod container_commands_socket;
pub mod feedback_commands_socket;
pub mod generic_sockets_with_parsers;
pub mod unix_socket;

pub static DAEMON_SOCKET: &'static str = "/tmp/rust.sock";

pub static CLIENT_SOCKET: &'static str = "/tmp/rust_client.sock";

pub enum ConnectionStatus {
    Running,
    Finished,
}

pub type ConnectionHandler = Box<dyn FnMut(Vec<u8>) -> Result<ConnectionStatus, String> + 'static>;
pub type ConnectionCommand = Vec<u8>;

pub trait SocketStream: Send {
    // Connect to the socket and return the file descriptor
    fn connect(&mut self) -> Result<i32, String>;
    fn send_command(&mut self, command: &ConnectionCommand) -> Result<(), String>;
}

pub trait SocketListener {
    fn prepare_socket(&mut self) -> Result<(), String>;
    fn listen(&mut self, handle_connection: &mut ConnectionHandler) -> Result<(), String>;
}

pub fn get_container_command_listener() -> ContainerCommandListener {
    Box::from(GenericCommandListener::new(Box::from(
        UnixSocketListener::new(DAEMON_SOCKET),
    )))
}

pub fn get_container_command_stream() -> ContainerCommandStream {
    Box::from(GenericCommandStream::new(Box::from(UnixSocketStream::new(
        DAEMON_SOCKET,
    ))))
}

pub fn get_client_socket_listener() -> FeedbackCommandListener {
    Box::from(GenericCommandListener::new(Box::from(
        UnixSocketListener::new(CLIENT_SOCKET),
    )))
}
pub fn get_client_socket_stream() -> FeedbackCommandStream {
    Box::from(GenericCommandStream::new(Box::from(UnixSocketStream::new(
        CLIENT_SOCKET,
    ))))
}

pub fn send_feedback(feedback_command: FeedbackCommand) -> Result<(), String> {
    let mut socket = get_client_socket_stream();
    socket.connect()?;
    socket.send_command(feedback_command)?;
    Ok(())
}
