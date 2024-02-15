use self::{
    container_commands_socket::{ContainerCommandListener, ContainerCommandStream},
    feedback_commands_socket::{FeedbackCommandListener, FeedbackCommandStream},
    generic_sockets_with_parsers::{GenericCommandListener, GenericCommandStream},
    unix_socket::{UnixSocketListener, UnixSocketStream},
};

use super::client_request::{ClientId, ClientResponse};

pub mod container_commands_socket;
pub mod feedback_commands_socket;
pub mod generic_sockets_with_parsers;
pub mod unix_socket;

pub static SOCKETS_PATH: &'static str = "/tmp/container-runtime/";

pub static DAEMON_SOCKET: &'static str = "/tmp/container-runtime/rust.sock";

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
    cleanup_sockets();
    Box::from(GenericCommandListener::new(Box::from(
        UnixSocketListener::new(DAEMON_SOCKET),
    )))
}

pub fn get_container_command_stream() -> ContainerCommandStream {
    Box::from(GenericCommandStream::new(Box::from(UnixSocketStream::new(
        DAEMON_SOCKET,
    ))))
}

pub fn get_client_socket_listener(client_id: ClientId) -> FeedbackCommandListener {
    Box::from(GenericCommandListener::new(Box::from(
        UnixSocketListener::new(client_id.get_id()),
    )))
}
pub fn get_client_socket_stream(client: ClientId) -> FeedbackCommandStream {
    Box::from(GenericCommandStream::new(Box::from(UnixSocketStream::new(
        client.get_id(),
    ))))
}

pub fn cleanup_sockets() {
    std::fs::remove_dir_all(SOCKETS_PATH).unwrap_or_default();
    std::fs::create_dir_all(SOCKETS_PATH).unwrap_or_default();
}

pub fn send_feedback(client_response: ClientResponse) -> Result<(), String> {
    let mut socket = get_client_socket_stream(client_response.client_id);
    socket.connect()?;
    socket.send_command(client_response.command)?;
    Ok(())
}
