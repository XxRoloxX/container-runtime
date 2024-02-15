use crate::controllers::Controller;
use crate::{container::Container, runner::Runner};
use container_runtime::common::client_request::ClientRequest;
use container_runtime::common::image::Image;
use container_runtime::common::runtime_commands::ContainerCommand;
use container_runtime::common::sockets::ConnectionStatus;

pub struct StartContainerController<'a> {
    runner: &'a mut Runner,
}

impl StartContainerController<'_> {
    pub fn new<'a>(runner: &'a mut Runner) -> StartContainerController<'a> {
        StartContainerController { runner }
    }
}
impl Controller<ClientRequest> for StartContainerController<'_> {
    fn handle_connection(&mut self, request: ClientRequest) -> Result<ConnectionStatus, String> {
        match request.command {
            ContainerCommand::Start {
                container_id,
                image,
                command,
                args,
            } => unsafe {
                self.runner.start_container(
                    Container::new(container_id.clone(), Image::new(image), command, args),
                    request.client_id,
                )?;
            },
            _ => {
                return Err(format!(
                    "Command not supported by this controller {}",
                    request.command
                ));
            }
        }

        Ok(ConnectionStatus::Running)
    }
}
