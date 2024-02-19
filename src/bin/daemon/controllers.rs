use container_runtime::common::sockets::ConnectionStatus;

pub mod build_image_controller;
pub mod list_images_controller;
pub mod list_running_containers_controller;
pub mod start_container_controller;
pub mod stop_container_controller;

pub trait Controller<T> {
    fn handle_connection(&mut self, buf: T) -> Result<ConnectionStatus, String>;
}
