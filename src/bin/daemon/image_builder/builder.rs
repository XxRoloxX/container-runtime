use container_runtime::common::{
    client_request::{ClientId, ClientResponse},
    feedback_commands::FeedbackCommand,
    filesystem::{change_current_dir, clear_directory, copy_directory},
    image::Image,
    process::wait_for_child_process,
    sockets::send_feedback,
};
use log::{error, info};
use nix::unistd::{fork, ForkResult};
use std::{path::PathBuf, process::Command, thread};

use crate::image_builder::parser::DockerfileInstruction;

use super::parser::parse_dockerfile;

pub struct ImageBuilder {}

impl ImageBuilder {
    pub fn new() -> ImageBuilder {
        ImageBuilder {}
    }

    pub fn build(dockerfile: &str, image: &Image, client_id: ClientId) -> Result<(), String> {
        let instructions = parse_dockerfile(dockerfile)?;
        ImageBuilder::prepare_image_directory(&image)?;
        info!("Image {} built successfully", image.id);

        for instruction in instructions {
            match instruction {
                DockerfileInstruction::RUN(command) => unsafe {
                    ImageBuilder::run_command(&image, command)?;
                },
                DockerfileInstruction::COPY(source, destination) => {
                    ImageBuilder::copy_file(&image, dockerfile.to_string(), source, destination)?;
                }
                DockerfileInstruction::FROM(source_image_id) => {
                    ImageBuilder::copy_image(&image, source_image_id)?;
                }
                DockerfileInstruction::ENTRYPOINT(entrypoint) => {
                    ImageBuilder::add_entrypoint(&image, entrypoint)?;
                }
            }
        }

        let response = ClientResponse::new(
            client_id,
            FeedbackCommand::ImageBuilt {
                image: Image::new(image.id.clone()),
            },
        );

        send_feedback(response)?;

        Ok(())
    }

    fn add_entrypoint(image: &Image, entrypoint: String) -> Result<(), String> {
        let mut entrypoints = image.get_entrypoints()?;
        entrypoints.add_entrypoint(entrypoint);
        image.set_entrypoints(entrypoints)?;

        Ok(())
    }

    fn copy_image(image: &Image, source_image_id: String) -> Result<(), String> {
        let source_image = Image::new(source_image_id);
        source_image.clone_image_contents(image)?;
        Ok(())
    }

    unsafe fn run_command(image: &Image, command: String) -> Result<(), String> {
        // NOTE: This is a workaround to finish the chroot process
        let image_clone = image.clone();
        let handle = thread::spawn(move || match fork() {
            Ok(ForkResult::Parent { child, .. }) => {
                wait_for_child_process(child);
                info!("Command {} executed successfully", command);
            }
            Ok(ForkResult::Child { .. }) => {
                match ImageBuilder::execute_command_in_isolation(&image_clone, command) {
                    Ok(_) => {}
                    Err(e) => error!("Failed to execute command: {}", e),
                }
            }
            Err(e) => error!("Failed to fork: {}", e),
        });
        handle.join().unwrap();
        Ok(())
    }

    // Run this function only if it is separated from the main process (via fork or unshare)
    fn execute_command_in_isolation(image: &Image, command: String) -> Result<(), String> {
        let image_path = image.get_filesystem_path()?;

        change_current_dir(image_path.as_str())
            .map_err(|e| format!("Failed to change dir: {}", e))?;

        nix::unistd::chroot(image_path.as_str()).map_err(|e| format!("Failed to chroot: {}", e))?;
        nix::unistd::chdir("/").map_err(|e| format!("Failed to chdir: {}", e))?;

        let output = Command::new("sh")
            .arg("-c")
            .arg(command.clone())
            .output()
            .map_err(|e| format!("Failed to execute command: {}", e))?;

        output
            .stdout
            .iter()
            .for_each(|byte| print!("{}", *byte as char));

        Ok(())
    }

    fn copy_file(
        image: &Image,
        dockerfile: String,
        source: String,
        destination: String,
    ) -> Result<(), String> {
        let image_path = image.get_filesystem_path()?;

        // Make the source path relative to the Dockerfile
        let source_path = PathBuf::from(dockerfile)
            .parent()
            .ok_or("Failed to get parent directory")?
            .join(source);

        let destination_path = format!("{}/{}", image_path, destination);
        copy_directory(source_path.to_str().unwrap(), destination_path.as_str())?;
        Ok(())
    }

    fn prepare_image_directory(image: &Image) -> Result<(), String> {
        clear_directory(image.get_image_path()?.as_str())?;
        Ok(())
    }
}
