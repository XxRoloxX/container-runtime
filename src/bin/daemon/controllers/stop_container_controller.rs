use crate::controllers::Controller;
use crate::runner::Runner;
use container_runtime::common::client_request::ClientRequest;
use container_runtime::common::commands::runtime_commands::ContainerCommand;
use container_runtime::common::sockets::ConnectionStatus;
use log::info;

pub struct StopContainerController<'a> {
    runner: &'a mut Runner,
}

impl StopContainerController<'_> {
    pub fn new<'a>(runner: &'a mut Runner) -> StopContainerController<'a> {
        StopContainerController { runner }
    }
}
impl Controller<ClientRequest> for StopContainerController<'_> {
    fn handle_connection(&mut self, request: ClientRequest) -> Result<ConnectionStatus, String> {
        match request.command {
            ContainerCommand::Stop { container_id } => {
                info!("Stopping container {}", container_id);
                self.runner.stop_container(container_id)?;
            }
            _ => {
                return Err(format!(
                    "Command not supported by this controller {}",
                    request.command
                ));
            }
        }

        Ok(ConnectionStatus::Finished)
    }
}
