pub mod build_image_controller;
pub mod start_container_controller;

pub trait Controller<T> {
    fn handle_connection(&mut self, buf: T) -> Result<(), String>;
}
