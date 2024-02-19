use container_runtime::common::client_request::ClientRequest;
use container_runtime::common::process::ignore_process_termination;
use container_runtime::common::sockets::generic_sockets_with_parsers::CommandHandler;
use container_runtime::common::sockets::sockets_with_parsers::container_commands_socket::ContainerCommandListener;
use log::info;

use crate::router::route_message;
use crate::runner::Runner;

pub fn run_server(mut socket: ContainerCommandListener) -> Result<(), String> {
    let mut runner = Runner::new(4);
    info!("Server started");
    socket.prepare_socket()?;
    let router: CommandHandler<ClientRequest> =
        Box::from(move |buf| route_message(&mut runner, buf));
    // ignore_process_termination()?;
    socket.listen(router)?;
    Ok(())
}
