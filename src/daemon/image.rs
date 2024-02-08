use std::path::Path;

use crate::common::process::get_install_path;

pub struct Image {
    id: String,
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
}
