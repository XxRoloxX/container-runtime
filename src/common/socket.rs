use super::{
    commands::ContainerCommand,
    unix_socket::{UnixSocketListener, UnixSocketStream},
};

pub trait SocketStream {
    fn connect(&mut self) -> Result<(), String>;
    fn send_command(&mut self, command: &ContainerCommand) -> Result<(), String>;
}

pub trait SocketListener {
    fn prepare_socket(&mut self) -> Result<(), String>;
    fn listen(&mut self, handle_connection: Box<dyn Fn(Vec<u8>)>) -> Result<(), String>;
}

pub fn get_socket_listener() -> Box<dyn SocketListener> {
    Box::new(UnixSocketListener::new())
}

pub fn get_socket_stream() -> Box<dyn SocketStream> {
    Box::new(UnixSocketStream::new())
}
