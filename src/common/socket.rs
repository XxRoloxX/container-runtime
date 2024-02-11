use super::{
    commands::ContainerCommand,
    unix_socket::{UnixSocketListener, UnixSocketStream},
};

pub static DAEMON_SOCKET: &'static str = "/tmp/rust.sock";

pub static CLIENT_SOCKET: &'static str = "/tmp/rust_client.sock";

pub trait SocketStream: Send {
    // Connect to the socket and return the file descriptor
    fn connect(&mut self) -> Result<i32, String>;
    fn send_command(&mut self, command: &ContainerCommand) -> Result<(), String>;
}

pub type ConnectionHandler = Box<dyn FnMut(Vec<u8>) + 'static>;

pub trait SocketListener {
    fn prepare_socket(&mut self) -> Result<(), String>;
    fn listen(&mut self, handle_connection: &mut ConnectionHandler) -> Result<(), String>;
}

pub fn get_daemon_socket_listener() -> Box<dyn SocketListener> {
    Box::new(UnixSocketListener::new(DAEMON_SOCKET))
}

pub fn get_daemon_socket_stream() -> Box<dyn SocketStream> {
    Box::new(UnixSocketStream::new(DAEMON_SOCKET))
}

pub fn get_client_socket_listener() -> Box<dyn SocketListener> {
    Box::new(UnixSocketListener::new(CLIENT_SOCKET))
}

pub fn get_client_socket_stream() -> Box<dyn SocketStream> {
    Box::new(UnixSocketStream::new(CLIENT_SOCKET))
}
