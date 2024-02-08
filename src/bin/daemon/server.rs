use container_runtime::common::socket::SOCKET_PATH;
use std::io::Read;
use std::os::unix::net::UnixListener;

use crate::container_runner::ContainerRunner;
use crate::controller::ContainerController;

pub fn run_server() -> Result<(), String> {
    nix::unistd::unlink(SOCKET_PATH).unwrap_or_default();
    let runner = ContainerRunner::new(4);
    let controller = ContainerController::new(runner);

    let listener =
        UnixListener::bind(SOCKET_PATH).map_err(|e| format!("Failed to create listener: {}", e))?;

    for stream in listener.incoming() {
        let mut connection = stream.map_err(|e| format!("Connection faield {}", e))?;
        println!("Got connection");
        let mut buf: [u8; 100] = [0u8; 100];

        connection
            .read(&mut buf)
            .map_err(|e| format!("Failed to read data: {}", e))?;

        controller.handle_connection(Vec::from(buf));
    }
    Ok(())
}
