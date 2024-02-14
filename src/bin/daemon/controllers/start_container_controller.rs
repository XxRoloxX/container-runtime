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
impl Controller<ContainerCommand> for StartContainerController<'_> {
    fn handle_connection(&mut self, command: ContainerCommand) -> Result<(), String> {
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
