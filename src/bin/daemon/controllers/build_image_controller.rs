use crate::controllers::Controller;
use crate::image_builder::builder::ImageBuilder;
use container_runtime::common::client_request::ClientRequest;
use container_runtime::common::image::Image;
use container_runtime::common::runtime_commands::ContainerCommand;
use container_runtime::common::sockets::ConnectionStatus;
use log::info;

pub struct BuildImageController {
    // runner: Runner,
}

impl BuildImageController {
    pub fn new() -> BuildImageController {
        BuildImageController {}
    }
}
impl Controller<ClientRequest> for BuildImageController {
    fn handle_connection(&mut self, request: ClientRequest) -> Result<ConnectionStatus, String> {
        let command = request.command;
        match command {
            ContainerCommand::Build {
                dockerfile,
                image_id,
            } => {
                ImageBuilder::build(
                    dockerfile.to_str().unwrap(),
                    &Image::new(image_id.clone()),
                    request.client_id,
                )?;
                info!("Image {} built successfully", image_id);
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
