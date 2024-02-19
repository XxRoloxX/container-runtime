use container_runtime::common::{
    client_request::ClientRequest,
    commands::runtime_commands::{ContainerCommand, ImageCommand},
    sockets::ConnectionStatus,
};

use crate::{
    controllers::{
        build_image_controller::BuildImageController, list_images_controller::ListImagesController,
        start_container_controller::StartContainerController, Controller,
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
        ContainerCommand::Image(ImageCommand::List) => Box::from(ListImagesController::new()),
    }
}
