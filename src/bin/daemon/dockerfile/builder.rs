use std::process::Command;

use container_runtime::common::{
    filesystem::{clear_directory, copy_directory},
    image::Image,
    thread_pool::ThreadPool,
};
use log::info;

use crate::dockerfile::parser::DockerfileInstruction;

use super::parser::parse_dockerfile;

pub struct ImageBuilder {
    pool: ThreadPool,
}

impl ImageBuilder {
    pub fn new(pool_size: usize) -> ImageBuilder {
        ImageBuilder {
            pool: ThreadPool::new(pool_size),
        }
    }

    pub fn build(dockerfile: &str, image: &Image) -> Result<(), String> {
        let instructions = parse_dockerfile(dockerfile)?;
        ImageBuilder::prepare_image_directory(&image)?;
        info!("Image {} built successfully", image.id);

        for instruction in instructions {
            match instruction {
                DockerfileInstruction::RUN(command) => {
                    ImageBuilder::run_command(&image, command)?;
                }
                DockerfileInstruction::COPY(source, destination) => {
                    ImageBuilder::copy_file(&image, source, destination)?;
                }
            }
        }

        Ok(())
    }

    fn run_command(image: &Image, command: String) -> Result<(), String> {
        let image_path = image.get_image_path()?;
        nix::unistd::chroot(image_path.as_str())
            .map_err(|e| format!("Couldn't chroot into {}: {}", image_path, e))?;
        nix::unistd::chdir("/").map_err(|e| format!("Couldn't chdir into /: {}", e))?;
        let command_with_exit = format!("{} && exit", command);
        let output = Command::new(command_with_exit)
            .output()
            .map_err(|e| format!("Couldn't execute command {} : {}", command, e))?;

        output
            .stdout
            .iter()
            .for_each(|byte| print!("{}", *byte as char));
        Ok(())
    }

    fn copy_file(image: &Image, source: String, destination: String) -> Result<(), String> {
        let image_path = image.get_image_path()?;
        let destination_path = format!("{}/{}", image_path, destination);
        copy_directory(source.as_str(), destination_path.as_str())?;
        Ok(())
    }

    fn copy_base_image(image: &Image) -> Result<(), String> {
        let base_image = Image::new("base".to_string());
        let base_image_path = base_image.get_image_path()?;
        let destination_path = image.get_image_path()?;
        copy_directory(base_image_path.as_str(), destination_path.as_str())?;
        Ok(())
    }

    fn prepare_image_directory(image: &Image) -> Result<(), String> {
        clear_directory(image.get_image_path()?.as_str())?;
        ImageBuilder::copy_base_image(image)?;
        Ok(())
    }
}
