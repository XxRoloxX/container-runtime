use crate::controllers::Controller;
use crate::image_builder::builder::ImageBuilder;
use container_runtime::common::commands::ContainerCommand;
use container_runtime::common::image::Image;
use log::info;

pub struct BuildImageController {
    // runner: Runner,
}

impl BuildImageController {
    pub fn new() -> BuildImageController {
        BuildImageController {}
    }
}
impl Controller<ContainerCommand> for BuildImageController {
    fn handle_connection(&self, command: ContainerCommand) -> Result<(), String> {
        match command {
            ContainerCommand::Build {
                dockerfile,
                image_id,
            } => {
                ImageBuilder::build(dockerfile.as_str(), &Image::new(image_id.clone()))?;
                info!("Image {} built successfully", image_id);
            }
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
