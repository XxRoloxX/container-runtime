use container_runtime::common::socket::SocketListener;

use crate::controller::ContainerController;
use crate::runner::Runner;

pub fn run_server(mut socket: Box<dyn SocketListener>) -> Result<(), String> {
    let runner = Runner::new(4);
    let controller = ContainerController::new(runner);
    socket.prepare_socket()?;
    socket.listen(Box::from(move |buf| controller.handle_connection(buf)))?;
    Ok(())
}
