use container_runtime::common::commands::ContainerCommand;
pub mod build_image_controller;
pub mod start_container_controller;

pub trait Controller {
    fn handle_connection(&self, buf: Vec<u8>) -> Result<(), String>;
}

pub fn parse_command(buf: &Vec<u8>) -> Result<ContainerCommand, String> {
    let stringified_data =
        String::from_utf8(buf.clone()).map_err(|e| format!("Couldn't parse command {}", e))?;

    let command = serde_json::from_str(stringified_data.as_ref())
        .map_err(|e| format!("Couldn't parse the Command {}", e))?;
    Ok(command)
}
