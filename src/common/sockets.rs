use self::{
    container_commands_socket::{ContainerCommandListener, ContainerCommandStream},
    unix_socket::{UnixSocketListener, UnixSocketStream},
};

pub mod unix_socket;

pub mod container_commands_socket;
pub mod feedback_commands_socket;

pub static DAEMON_SOCKET: &'static str = "/tmp/rust.sock";

pub static CLIENT_SOCKET: &'static str = "/tmp/rust_client.sock";

pub trait SocketStream: Send {
    // Connect to the socket and return the file descriptor
    fn connect(&mut self) -> Result<i32, String>;
    fn send_command(&mut self, command: &ConnectionCommand) -> Result<(), String>;
}

pub type ConnectionHandler = Box<dyn FnMut(Vec<u8>) + 'static>;
pub type ConnectionCommand = Vec<u8>;

pub trait SocketListener {
    fn prepare_socket(&mut self) -> Result<(), String>;
    fn listen(&mut self, handle_connection: &mut ConnectionHandler) -> Result<(), String>;
}

pub fn get_container_command_listener() -> Box<ContainerCommandListener> {
    Box::new(ContainerCommandListener::new(Box::from(
        UnixSocketListener::new(DAEMON_SOCKET),
    )))
}

pub fn get_container_command_stream() -> Box<ContainerCommandStream> {
    Box::new(ContainerCommandStream::new(Box::from(
        UnixSocketStream::new(DAEMON_SOCKET),
    )))
}

// pub fn get_client_socket_listener() -> Box<dyn SocketListener> {
//     Box::new(UnixSocketListener::new(CLIENT_SOCKET))
// }
//
// pub fn get_client_socket_stream() -> Box<dyn SocketStream> {
//     Box::new(UnixSocketStream::new(CLIENT_SOCKET))
// }
//
pub type CommandHandler<T> = Box<dyn FnMut(T) + 'static>;

pub trait SocketListenerWithParser<T> {
    fn prepare_socket(&mut self) -> Result<(), String>;
    fn listen(&mut self, handle_connection: CommandHandler<T>) -> Result<(), String>;
}

pub trait SocketStreamWithParser<T> {
    fn connect(&mut self) -> Result<i32, String>;
    fn send_command(&mut self, command: &T) -> Result<(), String>;
}
