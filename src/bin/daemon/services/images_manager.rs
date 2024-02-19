use std::fs;

use container_runtime::common::image::Image;

pub fn list_images() -> Result<Vec<String>, String> {
    let images_path = Image::get_images_path()?;
    let images_dir = fs::read_dir(images_path).map_err(|e| e.to_string())?;
    Ok(images_dir
        .map(|entry| match entry {
            Ok(dir) => dir.file_name().into_string().unwrap(),
            Err(_) => "".to_string(),
        })
        .filter(|dir| dir != "")
        .collect())
}
