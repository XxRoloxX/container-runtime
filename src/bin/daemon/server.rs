use container_runtime::common::sockets::container_commands_socket::{
    ContainerCommandHandler, ContainerCommandListener,
};

use crate::router::route_message;
use crate::runner::Runner;

pub fn run_server(mut socket: Box<ContainerCommandListener>) -> Result<(), String> {
    let mut runner = Runner::new(4);
    socket.prepare_socket()?;
    let router: ContainerCommandHandler = Box::from(move |buf| route_message(&mut runner, buf));
    socket.listen(router)?;
    Ok(())
}
