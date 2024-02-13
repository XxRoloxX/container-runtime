use crate::common::commands::ContainerCommand;

use super::generic_sockets_with_parsers::{SocketListenerWithParser, SocketStreamWithParser};

pub type ContainerCommandStream = Box<dyn SocketStreamWithParser<ContainerCommand>>;
pub type ContainerCommandListener = Box<dyn SocketListenerWithParser<ContainerCommand>>;
