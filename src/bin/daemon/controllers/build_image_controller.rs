use super::parse_command;
use crate::controllers::Controller;
use crate::dockerfile::builder::ImageBuilder;
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
impl Controller for BuildImageController {
    fn handle_connection(&self, buf: Vec<u8>) -> Result<(), String> {
        let command = parse_command(&buf)?;

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
