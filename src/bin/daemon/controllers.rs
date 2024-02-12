use container_runtime::common::commands::ContainerCommand;
pub mod build_image_controller;
pub mod start_container_controller;

pub trait Controller<T> {
    fn handle_connection(&self, buf: T) -> Result<(), String>;
}

// pub trait ContainerCommandController = Controller<ContainerCommand>;
