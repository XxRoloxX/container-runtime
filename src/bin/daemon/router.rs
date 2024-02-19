use container_runtime::common::{
    client_request::ClientRequest,
    commands::runtime_commands::{ContainerCommand, ImageCommand},
    sockets::ConnectionStatus,
};

use crate::{
    controllers::{
        build_image_controller::BuildImageController, list_images_controller::ListImagesController,
        list_running_containers_controller::ListRunningContainersController,
        start_container_controller::StartContainerController,
        stop_container_controller::StopContainerController, Controller,
    },
    runner::Runner,
};

pub fn route_message(
    runner: &mut Runner,
    request: ClientRequest,
) -> Result<ConnectionStatus, String> {
    let command = request.command.clone();

    let mut controller = match_command_to_controller(runner, command);

    let status = controller.handle_connection(request)?;
    Ok(status)
}

pub fn match_command_to_controller<'a>(
    runner: &'a mut Runner,
    command: ContainerCommand,
) -> Box<dyn Controller<ClientRequest> + 'a> {
    match command {
        ContainerCommand::Build { .. } => Box::from(BuildImageController::new()),
        ContainerCommand::Start { .. } => Box::from(StartContainerController::new(runner)),
        ContainerCommand::Stop { .. } => Box::from(StopContainerController::new(runner)),
        ContainerCommand::Image(ImageCommand::List) => Box::from(ListImagesController::new()),
        ContainerCommand::List {} => Box::from(ListRunningContainersController::new(runner)),
    }
}
