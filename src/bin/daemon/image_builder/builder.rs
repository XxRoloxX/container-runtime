use std::process::Command;

use container_runtime::common::{
    filesystem::{clear_directory, copy_directory},
    image::Image,
    process::wait_for_child_process,
};
use log::info;
use nix::unistd::{fork, ForkResult};

use crate::image_builder::parser::DockerfileInstruction;

use super::parser::parse_dockerfile;

pub struct ImageBuilder {}

impl ImageBuilder {
    pub fn new() -> ImageBuilder {
        ImageBuilder {}
    }

    pub fn build(dockerfile: &str, image: &Image) -> Result<(), String> {
        let instructions = parse_dockerfile(dockerfile)?;
        ImageBuilder::prepare_image_directory(&image)?;
        info!("Image {} built successfully", image.id);

        for instruction in instructions {
            match instruction {
                DockerfileInstruction::RUN(command) => unsafe {
                    ImageBuilder::run_command(&image, command)?;
                },
                DockerfileInstruction::COPY(source, destination) => {
                    ImageBuilder::copy_file(&image, source, destination)?;
                }
                DockerfileInstruction::FROM(source_image_id) => {
                    ImageBuilder::copy_image(&image, source_image_id)?;
                }
            }
        }

        Ok(())
    }

    fn copy_image(image: &Image, source_image_id: String) -> Result<(), String> {
        let source_image = Image::new(source_image_id);
        source_image.clone_image_contents(image)?;
        Ok(())
    }

    unsafe fn run_command(image: &Image, command: String) -> Result<(), String> {
        let image_path = image.get_image_path()?;

        match fork() {
            Ok(ForkResult::Parent { child, .. }) => {
                wait_for_child_process(child);
                info!("Command {} executed successfully", command);
            }
            Ok(ForkResult::Child { .. }) => {
                nix::unistd::chroot(image_path.as_str())
                    .map_err(|e| format!("Couldn't chroot into {}: {}", image_path, e))?;
                nix::unistd::chdir("/").map_err(|e| format!("Couldn't chdir into /: {}", e))?;

                //TODO: Process is not exiting chroot environment
                let command_with_exit = format!("{} && exit", command);

                info!("Executing command: {}", command);
                let output = Command::new("sh")
                    .arg("-c")
                    .arg(command_with_exit)
                    .output()
                    .map_err(|e| format!("Couldn't execute command {} : {}", command, e))?;

                output
                    .stdout
                    .iter()
                    .for_each(|byte| print!("{}", *byte as char));
            }
            Err(e) => return Err(format!("Failed to fork: {}", e)),
        }
        Ok(())
    }

    fn copy_file(image: &Image, source: String, destination: String) -> Result<(), String> {
        let image_path = image.get_image_path()?;
        let destination_path = format!("{}/{}", image_path, destination);
        copy_directory(source.as_str(), destination_path.as_str())?;
        Ok(())
    }

    // fn copy_base_image(image: &Image) -> Result<(), String> {
    //     let base_image = Image::new("base".to_string());
    //     let base_image_path = base_image.get_image_path()?;
    //     let destination_path = image.get_image_path()?;
    //     copy_directory(base_image_path.as_str(), destination_path.as_str())?;
    //     Ok(())
    // }

    fn prepare_image_directory(image: &Image) -> Result<(), String> {
        clear_directory(image.get_image_path()?.as_str())?;
        Ok(())
    }
}
