use container_runtime::common::commands::ContainerCommand;
use log::error;

use crate::{
    controllers::{
        build_image_controller::BuildImageController, parse_command,
        start_container_controller::StartContainerController, Controller,
    },
    runner::Runner,
};

pub fn route_message(runner: &mut Runner, buf: Vec<u8>) {
    let command = match parse_command(&buf) {
        Ok(command) => command,
        Err(e) => {
            error!("Error parsing command {}", e);
            return;
        }
    };

    let controller: Box<dyn Controller> = match command {
        ContainerCommand::Build { .. } => Box::from(BuildImageController::new()),
        ContainerCommand::Start { .. } => Box::from(StartContainerController::new(runner)),
        _ => {
            error!("Command not supported by router");
            return;
        }
    };

    if let Err(err) = controller.handle_connection(buf) {
        error!("Error handling connection {}", err);
    }
}
