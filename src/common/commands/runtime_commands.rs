use std::{
    fmt::{self, Display, Formatter},
    path::PathBuf,
};

use clap::Subcommand;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkConfiguration {
    Host,
    None,
}

impl Default for NetworkConfiguration {
    fn default() -> Self {
        NetworkConfiguration::Host
    }
}

impl From<&str> for NetworkConfiguration {
    fn from(s: &str) -> Self {
        match s {
            "host" => NetworkConfiguration::Host,
            "none" => NetworkConfiguration::None,
            _ => NetworkConfiguration::Host,
        }
    }
}

#[derive(Subcommand, Serialize, Deserialize, Clone, Debug)]
pub enum ImageCommand {
    /// List all images
    List,
}

#[derive(Subcommand, Serialize, Deserialize, Clone, Debug)]
pub enum ContainerCommand {
    #[command(subcommand)]
    Image(ImageCommand),
    /// Start a container
    Start {
        /// The container id
        container_id: String,

        /// The image to use
        image: String,

        // The network configuration
        network: Option<NetworkConfiguration>,

        /// The command to run as the root process
        command: String,

        /// The arguments to pass to the command
        args: Vec<String>,
    },
    Stop {
        /// The container id
        container_id: String,
    },
    Build {
        /// The image id
        image_id: String,
        /// The path to the Dockerfile (canonaize it first to get the absolute path)
        dockerfile: PathBuf,
    },

    // List all running containers
    List {},
}

impl ContainerCommand {
    pub fn canonize_paths(&mut self) {
        match self {
            ContainerCommand::Build { dockerfile, .. } => {
                *dockerfile = dockerfile.canonicalize().unwrap();
            }
            _ => {}
        }
    }
}

impl Display for ContainerCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ContainerCommand::Start {
                container_id,
                network,
                image,
                command,
                args,
            } => write!(
                f,
                "Start container {} with network {:?} and image {} and command {} and args {:?}",
                container_id,
                network.clone().unwrap_or(Default::default()),
                image,
                command,
                args
            ),
            ContainerCommand::Build {
                image_id,
                dockerfile,
            } => write!(
                f,
                "Build image {} with Dockerfile {}",
                image_id,
                dockerfile.to_str().unwrap()
            ),
            ContainerCommand::Image(image_command) => {
                write!(f, "Image command {:?}", image_command)
            }
            ContainerCommand::Stop { container_id } => write!(f, "Stop container {}", container_id),
            ContainerCommand::List {} => write!(f, "List running containers"),
        }
    }
}
