use crate::router::route_message;
use crate::runner::Runner;
use container_runtime::common::socket::{ConnectionHandler, SocketListener};

pub fn run_server(mut socket: Box<dyn SocketListener>) -> Result<(), String> {
    let mut runner = Runner::new(4);
    socket.prepare_socket()?;
    let mut router: ConnectionHandler = Box::from(move |buf| route_message(&mut runner, buf));
    socket.listen(&mut router)?;
    Ok(())
}
