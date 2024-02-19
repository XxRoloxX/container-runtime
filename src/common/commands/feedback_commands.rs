use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

use crate::common::image::Image;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum FeedbackCommand {
    ImageBuilt { image: Image },
    ContainerStarted { pid: i32, name: String },
    ContainerExited { pid: i32, name: String },
    Content(String),
}

impl Display for FeedbackCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FeedbackCommand::ImageBuilt { image } => write!(f, "Image built: {:?}", image),
            FeedbackCommand::ContainerStarted { pid, name } => {
                write!(f, "Container started with pid {} and name {}", pid, name)
            }
            FeedbackCommand::ContainerExited { pid, name } => {
                write!(f, "Container exited with pid {} and name {}", pid, name)
            }
            FeedbackCommand::Content(content) => write!(f, "Content: {}", content),
        }
    }
}
