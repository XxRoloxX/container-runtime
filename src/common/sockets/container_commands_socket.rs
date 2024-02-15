use crate::common::client_request::ClientRequest;

use super::generic_sockets_with_parsers::{SocketListenerWithParser, SocketStreamWithParser};

pub type ContainerCommandStream = Box<dyn SocketStreamWithParser<ClientRequest>>;
pub type ContainerCommandListener = Box<dyn SocketListenerWithParser<ClientRequest>>;
