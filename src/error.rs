use std::{
    error::Error,
    fmt::{self, Display},
};

#[derive(Debug)]
pub enum WebsocketEventError {
    UnrecognizedCommand,
    ParseError,
}

impl Display for WebsocketEventError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WebsocketEventError::ParseError => write!(f, "failed to parse websocket event"),
            WebsocketEventError::UnrecognizedCommand => {
                write!(f, "failed to parse websocket command")
            }
        }
    }
}

impl Error for WebsocketEventError {}
