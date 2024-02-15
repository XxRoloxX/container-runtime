use container_runtime::common::client_request::ClientRequest;
use container_runtime::common::sockets::container_commands_socket::ContainerCommandListener;
use container_runtime::common::sockets::generic_sockets_with_parsers::CommandHandler;

use crate::router::route_message;
use crate::runner::Runner;

pub fn run_server(mut socket: ContainerCommandListener) -> Result<(), String> {
    let mut runner = Runner::new(4);
    socket.prepare_socket()?;
    let router: CommandHandler<ClientRequest> =
        Box::from(move |buf| route_message(&mut runner, buf));
    socket.listen(router)?;
    Ok(())
}
