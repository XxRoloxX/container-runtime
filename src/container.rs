use std::path::{Path, PathBuf};

use crate::{
    image::Image,
    rootfs::{clear_directory, mount_overlayfs},
    unshare::get_install_path,
};

pub struct Container {
    id: String,
    image: Box<Image>,
    pub command: String,
    pub args: Vec<String>,
}

impl Container {
    pub fn new(id: String, image: Image, command: String, args: Vec<String>) -> Container {
        Container {
            id,
            image: Box::from(image),
            command,
            args,
        }
    }
    fn get_inner_containers_path(dirname: &str) -> Result<PathBuf, String> {
        let container_path = Self::get_containers_path()?;
        Ok(Path::new(&container_path).join(dirname))
    }
    fn get_inner_overlay_path(&self, dirname: &str) -> Result<String, String> {
        let overlay_path = self.get_overlayfs_path()?;
        let overlay_path_builder = Path::new(&overlay_path).join(dirname);
        let overlay_path = overlay_path_builder.to_str().ok_or_else(|| {
            format!(
                "Failed to access overlay path of {} on {}",
                self.id, overlay_path
            )
        })?;
        Ok(overlay_path.to_string())
    }
    fn prepare_container_directories(&self) -> Result<(), String> {
        let work_layer_path = self.get_work_overlayfs_path()?;
        let upper_layer_path = self.get_upper_overlayfs_path()?;
        let merged_layer_path = self.get_merged_overlayfs_path()?;
        clear_directory(&work_layer_path)?;
        clear_directory(&upper_layer_path)?;
        clear_directory(&merged_layer_path)?;
        Ok(())
    }
    fn get_overlayfs_path(&self) -> Result<String, String> {
        let container_path = self.get_conatiner_path()?;
        let overlay_path = Path::new(&container_path).join("overlay");
        match overlay_path.to_str() {
            None => Err(format!(
                "Failed to access overlay path of {} on {}",
                self.id, container_path
            )),
            Some(path) => Ok(path.to_string()),
        }
    }
    fn get_lower_overlayfs_path(&self) -> Result<String, String> {
        self.image.get_image_path()
    }

    fn get_work_overlayfs_path(&self) -> Result<String, String> {
        self.get_inner_overlay_path("work")
    }
    fn get_merged_overlayfs_path(&self) -> Result<String, String> {
        self.get_inner_overlay_path("merged")
    }
    fn get_upper_overlayfs_path(&self) -> Result<String, String> {
        self.get_inner_overlay_path("upper")
    }
    pub fn mount_overlayfs(&self) -> Result<(), String> {
        self.prepare_container_directories()?;
        let lower = self.get_lower_overlayfs_path()?;
        let upper = self.get_upper_overlayfs_path()?;
        let work = self.get_work_overlayfs_path()?;
        let target = self.get_merged_overlayfs_path()?;
        mount_overlayfs(&lower, &upper, &work, &target);
        Ok(())
    }

    pub fn get_containers_path() -> Result<String, String> {
        let install_path = get_install_path()?;
        Ok(Path::new(&install_path)
            .join("containers")
            .to_str()
            .ok_or_else(|| "Failed to access containers path".to_string())?
            .to_string())
    }
    pub fn get_conatiner_path(&self) -> Result<String, String> {
        let containers_path = Self::get_containers_path()?;
        let container_path = Path::new(&containers_path).join(&self.id);
        match container_path.to_str() {
            None => Err(format!("Failed to access container path of {}", self.id)),
            Some(path) => Ok(path.to_string()),
        }
    }
}
