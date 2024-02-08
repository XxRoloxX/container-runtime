use clap::Subcommand;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Subcommand, Serialize, Deserialize)]
pub enum ContainerCommand {
    /// Start a container
    Start {
        /// The container id
        container_id: String,
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
