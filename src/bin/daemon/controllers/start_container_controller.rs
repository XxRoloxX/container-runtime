use super::parse_command;
use crate::controllers::Controller;
use crate::{container::Container, runner::Runner};
use container_runtime::common::commands::ContainerCommand;
use container_runtime::common::image::Image;

pub struct StartContainerController<'a> {
    runner: &'a mut Runner,
}

impl StartContainerController<'_> {
    pub fn new<'a>(runner: &'a mut Runner) -> StartContainerController<'a> {
        StartContainerController { runner }
    }
}
impl Controller for StartContainerController<'_> {
    fn handle_connection(&self, buf: Vec<u8>) -> Result<(), String> {
        let command = parse_command(&buf)?;

        match command {
            ContainerCommand::Start {
                container_id,
                image,
                command,
                args,
            } => unsafe {
                self.runner.start_container(Container::new(
                    container_id.clone(),
                    Image::new(image),
                    command,
                    args,
                ))?;
            },
            _ => {
                return Err(format!(
                    "Command not supported by this controller {}",
                    command
                ));
            }
        }

        Ok(())
    }
}
