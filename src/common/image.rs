use std::path::Path;

use log::info;
use nix::unistd::fork;
use serde::{Deserialize, Serialize};

use super::{filesystem::copy_directory, process::get_install_path};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Image {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryPoints {
    pub entrypoints: Vec<String>,
}

impl EntryPoints {
    pub fn new() -> EntryPoints {
        EntryPoints {
            entrypoints: Vec::new(),
        }
    }
    pub fn add_entrypoint(&mut self, entrypoint: String) {
        self.entrypoints.push(entrypoint);
    }

    pub unsafe fn execute_entrypoints(&self) -> Result<(), String> {
        match fork() {
            Ok(nix::unistd::ForkResult::Parent { child, .. }) => {
                nix::sys::wait::waitpid(child, None)
                    .map_err(|e| format!("Failed to wait for child: {}", e))?;
            }
            Ok(nix::unistd::ForkResult::Child) => {
                for entrypoint in &self.entrypoints {
                    info!("Executing entrypoint: {}", entrypoint);
                    let status = std::process::Command::new("sh")
                        .arg("-c")
                        .arg(entrypoint)
                        .status()
                        .map_err(|e| format!("Failed to execute entrypoint: {}", e))?;
                    if !status.success() {
                        return Err(format!("Entrypoint {} failed", entrypoint));
                    }
                }

                std::process::exit(0);
            }
            Err(e) => return Err(format!("Failed to fork: {}", e)),
        }
        Ok(())
    }
}

impl Image {
    pub fn new(id: String) -> Image {
        Image { id }
    }

    pub fn get_image_path(&self) -> Result<String, String> {
        let install_path = get_install_path()?;

        let image_path = Path::new(&install_path).join("images").join(&self.id);

        match image_path.to_str() {
            None => Err(format!(
                "Failed to access image path of {} on {}",
                self.id, install_path
            )),
            Some(path) => Ok(path.to_string()),
        }
    }
    pub fn get_filesystem_path(&self) -> Result<String, String> {
        let image_path = self.get_image_path()?;
        let filesystem_path = Path::new(&image_path).join("rootfs");

        match filesystem_path.to_str() {
            None => Err(format!(
                "Failed to access filesystem path of {} on {}",
                self.id, image_path
            )),
            Some(path) => Ok(path.to_string()),
        }
    }
    pub fn get_entrypoints(&self) -> Result<EntryPoints, String> {
        let entrypoint_path = self.get_entrypoint_path()?;
        let entrypoints = std::fs::read_to_string(entrypoint_path.as_str())
            .map_or(EntryPoints::new(), |content| {
                serde_json::from_str(&content).unwrap_or(EntryPoints::new())
            });
        Ok(entrypoints)
    }
    pub fn set_entrypoints(&self, entrypoints: EntryPoints) -> Result<(), String> {
        let entrypoint_path = self.get_entrypoint_path()?;
        std::fs::write(
            entrypoint_path,
            serde_json::to_string(&entrypoints).unwrap(),
        )
        .map_err(|e| format!("Failed to write entrypoints: {}", e))?;
        Ok(())
    }

    pub fn get_entrypoint_path(&self) -> Result<String, String> {
        let image_path = self.get_image_path()?;
        let entrypoint_path = Path::new(&image_path).join("entrypoints.json");

        match entrypoint_path.to_str() {
            None => Err(format!(
                "Failed to access entrypoint path of {} on {}",
                self.id, image_path
            )),
            Some(path) => Ok(path.to_string()),
        }
    }

    pub fn clone_image_contents(&self, destination_image: &Image) -> Result<(), String> {
        let image_path = self.get_filesystem_path()?;
        let dest_path = destination_image.get_filesystem_path()?;

        copy_directory(image_path.as_str(), dest_path.as_str())?;
        Ok(())
    }
}
