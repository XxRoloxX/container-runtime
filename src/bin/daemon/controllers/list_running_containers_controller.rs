use crate::controllers::Controller;
use crate::runner::Runner;
use container_runtime::common::client_request::{ClientRequest, ClientResponse};
use container_runtime::common::commands::feedback_commands::FeedbackCommand;
use container_runtime::common::commands::runtime_commands::ContainerCommand;
use container_runtime::common::sockets::{send_feedback, ConnectionStatus};

pub struct ListRunningContainersController<'a> {
    runner: &'a mut Runner,
}

impl ListRunningContainersController<'_> {
    pub fn new<'a>(runner: &'a mut Runner) -> ListRunningContainersController<'a> {
        ListRunningContainersController { runner }
    }
}
impl Controller<ClientRequest> for ListRunningContainersController<'_> {
    fn handle_connection(&mut self, request: ClientRequest) -> Result<ConnectionStatus, String> {
        match request.command {
            ContainerCommand::List {} => {
                let containers = self.runner.get_running_containers();
                let client_response = ClientResponse {
                    client_id: request.client_id,
                    command: FeedbackCommand::Content(format!(
                        "\nCONTAINERS\n{}",
                        containers
                            .iter()
                            .map(|c| c.to_string())
                            .collect::<Vec<String>>()
                            .join("\n")
                    )),
                };

                send_feedback(client_response)?;
            }

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
