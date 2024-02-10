use container_runtime::common::{
    filesystem::copy_directory, image::Image, thread_pool::ThreadPool,
};
use log::info;

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
        // parse_dockerfile(dockerfile)?;
        ImageBuilder::prepare_image_directory(&image)?;
        info!("Image {} built successfully", image.id);
        Ok(())
    }

    fn copy_base_image(image: &Image) -> Result<(), String> {
        let base_image = Image::new("debian".to_string());
        let base_image_path = base_image.get_image_path()?;
        let destination_path = image.get_image_path()?;
        copy_directory(base_image_path.as_str(), destination_path.as_str())?;
        Ok(())
    }

    fn prepare_image_directory(image: &Image) -> Result<(), String> {
        ImageBuilder::copy_base_image(image)?;
        Ok(())
    }
}
