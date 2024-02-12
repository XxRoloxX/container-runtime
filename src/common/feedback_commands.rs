use serde::{Deserialize, Serialize};

use super::image::Image;
#[derive(Serialize, Deserialize, Clone)]
pub enum FeedbackCommand {
    ImageBuilt { image: Image },
    ContainerStarted { pid: i32, name: String },
}
