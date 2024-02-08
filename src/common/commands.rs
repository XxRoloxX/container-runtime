use clap::Subcommand;
#[derive(Debug, Clone, Subcommand)]
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
