use container_runtime::common::commands::ContainerCommand;

use crate::{container::Container, image::Image, runner::Runner};

pub struct ContainerController {
    runner: Runner,
}

impl ContainerController {
    pub fn new(runner: Runner) -> ContainerController {
        ContainerController { runner }
    }

    pub fn handle_connection(&self, buf: Vec<u8>) {
        let stringified_data = String::from_utf8(buf);

        if let Err(err) = stringified_data {
            println!("Couldn't parse command: {}", err);
            return;
        }
        let command: Result<ContainerCommand, _> = serde_json::from_str(
            stringified_data
                .as_ref()
                .unwrap()
                .trim_matches(char::from(0)),
        );

        if let Err(err) = command {
            println!(
                "Couldn't covert {} to command: {}",
                stringified_data.unwrap(),
                err
            );
            return;
        }

        match command.unwrap() {
            ContainerCommand::Create {
                container_id,
                image,
            } => unsafe {
                println!("Creating container: {} from image: {}", container_id, image);
                self.runner.start_container(Container::new(
                    container_id,
                    Image::new(image),
                    "ps".to_string(),
                    vec!["aux".to_string()],
                ));
            },
            _ => {
                println!("Unsupported command");
                return;
            }
        };
    }
}
