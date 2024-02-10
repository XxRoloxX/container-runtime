use super::{
    commands::ContainerCommand,
    unix_socket::{UnixSocketListener, UnixSocketStream},
};

pub trait SocketStream {
    fn connect(&mut self) -> Result<(), String>;
    fn send_command(&mut self, command: &ContainerCommand) -> Result<(), String>;
}

pub type ConnectionHandler = Box<dyn FnMut(Vec<u8>) + 'static>;

pub trait SocketListener {
    fn prepare_socket(&mut self) -> Result<(), String>;
    fn listen(&mut self, handle_connection: &mut ConnectionHandler) -> Result<(), String>;
}

pub fn get_socket_listener() -> Box<dyn SocketListener> {
    Box::new(UnixSocketListener::new())
}

pub fn get_socket_stream() -> Box<dyn SocketStream> {
    Box::new(UnixSocketStream::new())
}
