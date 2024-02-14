use crate::common::feedback_commands::FeedbackCommand;

use super::generic_sockets_with_parsers::{SocketListenerWithParser, SocketStreamWithParser};

pub type FeedbackCommandStream = Box<dyn SocketStreamWithParser<FeedbackCommand>>;
pub type FeedbackCommandListener = Box<dyn SocketListenerWithParser<FeedbackCommand>>;
