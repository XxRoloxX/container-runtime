use std::fmt::{self, Display, Formatter};

use clap::Subcommand;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Subcommand, Serialize, Deserialize)]
pub enum ContainerCommand {
    /// Start a container
    Start {
        /// The container id
        container_id: String,
        image: String,
        command: String,
        args: Vec<String>,
    },
    Build {
        /// The image id
        image_id: String,
        /// The path to the Dockerfile
        dockerfile: String,
    },
    /// Stop a container
    Stop {
        /// The container id
        container_id: String,
    },
    /// Create a container
    Create {
        /// The container id
        container_id: String,

        /// The image to use
        image: String,
    },
}

impl Display for ContainerCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ContainerCommand::Start {
                container_id,
                image,
                command,
                args,
            } => write!(
                f,
                "Start container {} with image {} and command {} and args {:?}",
                container_id, image, command, args
            ),
            ContainerCommand::Build {
                image_id,
                dockerfile,
            } => write!(f, "Build image {} with Dockerfile {}", image_id, dockerfile),
            ContainerCommand::Stop { container_id } => {
                write!(f, "Stop container {}", container_id)
            }
            ContainerCommand::Create {
                container_id,
                image,
            } => {
                write!(f, "Create container {} with image {}", container_id, image)
            }
        }
    }
}
