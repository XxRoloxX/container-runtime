use std::path::Path;

use serde::{Deserialize, Serialize};

use super::{filesystem::copy_directory, process::get_install_path};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Image {
    pub id: String,
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

    pub fn clone_image_contents(&self, destination_image: &Image) -> Result<(), String> {
        let image_path = self.get_image_path()?;
        let dest_path = destination_image.get_image_path()?;

        copy_directory(image_path.as_str(), dest_path.as_str())?;
        Ok(())
    }
}
