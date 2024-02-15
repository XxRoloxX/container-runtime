use container_runtime::common::{runtime_commands::ContainerCommand, sockets::ConnectionStatus};

use crate::{
    controllers::{
        build_image_controller::BuildImageController,
        start_container_controller::StartContainerController, Controller,
    },
    runner::Runner,
};

pub fn route_message(
    runner: &mut Runner,
    command: ContainerCommand,
) -> Result<ConnectionStatus, String> {
    let mut controller: Box<dyn Controller<ContainerCommand>> = match command {
        ContainerCommand::Build { .. } => Box::from(BuildImageController::new()),
        ContainerCommand::Start { .. } => Box::from(StartContainerController::new(runner)),
        _ => {
            return Err("Command not supported".to_string());
        }
    };

    let status = controller.handle_connection(command)?;
    Ok(status)
}
