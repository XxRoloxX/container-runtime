use container_runtime::common::commands::ContainerCommand;
use log::error;

use crate::{
    controllers::{
        build_image_controller::BuildImageController,
        start_container_controller::StartContainerController, Controller,
    },
    runner::Runner,
};

pub fn route_message(runner: &mut Runner, command: ContainerCommand) {
    if !runner.is_output_socket_initialized() {
        runner.init_output_socket().unwrap();
    }

    let mut controller: Box<dyn Controller<ContainerCommand>> = match command {
        ContainerCommand::Build { .. } => Box::from(BuildImageController::new()),
        ContainerCommand::Start { .. } => Box::from(StartContainerController::new(runner)),
        _ => {
            error!("Command not supported by router");
            return;
        }
    };

    if let Err(err) = controller.handle_connection(command) {
        error!("Error handling connection {}", err);
    }
}
