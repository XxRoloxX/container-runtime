use crate::controllers::Controller;
use crate::services::images_manager;
use container_runtime::common::client_request::{ClientRequest, ClientResponse};
use container_runtime::common::commands::feedback_commands::FeedbackCommand;
use container_runtime::common::commands::runtime_commands::{ContainerCommand, ImageCommand};
use container_runtime::common::sockets::{send_feedback, ConnectionStatus};

pub struct ListImagesController {
    // runner: Runner,
}

impl ListImagesController {
    pub fn new() -> ListImagesController {
        ListImagesController {}
    }
}
impl Controller<ClientRequest> for ListImagesController {
    fn handle_connection(&mut self, request: ClientRequest) -> Result<ConnectionStatus, String> {
        let command = request.command;
        match command {
            ContainerCommand::Image(ImageCommand::List) => {
                let images = images_manager::list_images()?;
                let response = ClientResponse {
                    client_id: request.client_id,
                    command: FeedbackCommand::Content(images.join(", ")),
                };
                send_feedback(response)?;
            }
            _ => {
                return Err(format!(
                    "Command not supported by this controller {}",
                    command
                ));
            }
        }

        Ok(ConnectionStatus::Running)
    }
}
