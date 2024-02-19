use container_runtime::common::{
    commands::feedback_commands::FeedbackCommand, sockets::ConnectionStatus, strace::run_strace,
};
use log::{error, info};
use nix::unistd::Pid;

pub trait ResponseController {
    fn handle_response(&self, command: FeedbackCommand) -> Result<ConnectionStatus, String>;
}

pub struct ContainerStartedController;
pub struct ContainerExitedController;
pub struct ImageBuiltController;
pub struct ContentSentController;

impl ResponseController for ContainerStartedController {
    fn handle_response(&self, command: FeedbackCommand) -> Result<ConnectionStatus, String> {
        match command {
            FeedbackCommand::ContainerStarted { pid, .. } => {
                info!("Container started with pid {}", pid);
                run_strace(Pid::from_raw(pid));
                Ok(ConnectionStatus::Running)
            }
            _ => Err(format!("Invalid command {}", command).to_string()),
        }
    }
}

impl ResponseController for ContainerExitedController {
    fn handle_response(&self, command: FeedbackCommand) -> Result<ConnectionStatus, String> {
        match command {
            FeedbackCommand::ContainerExited { name, .. } => {
                info!("Container {} exited", name);
                Ok(ConnectionStatus::Finished)
            }
            _ => Err(format!("Invalid command {}", command).to_string()),
        }
    }
}

impl ResponseController for ImageBuiltController {
    fn handle_response(&self, command: FeedbackCommand) -> Result<ConnectionStatus, String> {
        match command {
            FeedbackCommand::ImageBuilt { image } => {
                info!("Image {} built", image.id);
                Ok(ConnectionStatus::Finished)
            }
            _ => Err(format!("Invalid command {}", command).to_string()),
        }
    }
}

impl ResponseController for ContentSentController {
    fn handle_response(&self, command: FeedbackCommand) -> Result<ConnectionStatus, String> {
        match command {
            FeedbackCommand::Content(data) => {
                info!("Recieved data: {}", data);
                Ok(ConnectionStatus::Finished)
            }
            _ => {
                error!("Command not supported");
                Ok(ConnectionStatus::Finished)
            }
        }
    }
}
