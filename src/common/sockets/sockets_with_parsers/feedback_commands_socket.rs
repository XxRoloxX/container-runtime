use crate::common::{
    commands::feedback_commands::FeedbackCommand,
    sockets::generic_sockets_with_parsers::{SocketListenerWithParser, SocketStreamWithParser},
};

pub type FeedbackCommandStream = Box<dyn SocketStreamWithParser<FeedbackCommand>>;
pub type FeedbackCommandListener = Box<dyn SocketListenerWithParser<FeedbackCommand>>;
