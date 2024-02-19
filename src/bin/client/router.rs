use container_runtime::common::{
    commands::feedback_commands::FeedbackCommand, sockets::ConnectionStatus,
};

use crate::controllers::{
    ContainerExitedController, ContainerStartedController, ContentSentController,
    ImageBuiltController, ResponseController,
};

pub fn route_feedback_command(command: FeedbackCommand) -> Result<ConnectionStatus, String> {
    let controller: Box<dyn ResponseController> = match command.clone() {
        FeedbackCommand::ContainerStarted { .. } => Box::from(ContainerStartedController {}),
        FeedbackCommand::ContainerExited { .. } => Box::from(ContainerExitedController {}),
        FeedbackCommand::ImageBuilt { .. } => Box::from(ImageBuiltController {}),
        FeedbackCommand::Content(_) => Box::from(ContentSentController {}),
    };

    controller.handle_response(command)
}
