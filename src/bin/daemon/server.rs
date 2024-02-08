use std::io::Read;
use std::os::unix::net::{UnixListener, UnixStream};
use std::{fs, path::Path};

use container_runtime::common::socket::SOCKET_PATH;

pub fn run_server() -> Result<(), String> {
    //Removed the fs::remove_file call
    // fs::remove_file(SOCKET_PATH).unwrap_or_default();
    nix::unistd::unlink(SOCKET_PATH).unwrap_or_default();

    let listener =
        UnixListener::bind(SOCKET_PATH).map_err(|e| format!("Failed to create listener: {}", e))?;

    for stream in listener.incoming() {
        let mut connection = stream.map_err(|e| format!("Connection faield {}", e))?;
        println!("Got connection");
        let mut buf: [u8; 100] = [0u8; 100];
        let data = connection.read(&mut buf);
        let stringified_data = String::from_utf8(Vec::from(buf)).unwrap();
        print!("Got data! {}", stringified_data);
    }
    Ok(())
}
