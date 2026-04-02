use std::{
    error::Error,
    fmt::{self, Display},
};

#[derive(Debug)]
pub enum WebsocketEventError {
    UnrecognizedCommand,
    ParseError,
    InvalidLength,
    InvalidUtf8,
    TruncatedBody,
    MissingTrailingCrlf,
    MissingLength,
}

impl Display for WebsocketEventError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WebsocketEventError::ParseError => write!(f, "failed to parse websocket event"),
            WebsocketEventError::UnrecognizedCommand => {
                write!(f, "failed to parse websocket command")
            }
            WebsocketEventError::InvalidLength => write!(f, "websocket frame was invalid length"),
            WebsocketEventError::InvalidUtf8 => write!(f, "websocket frame was invalid utf8"),
            WebsocketEventError::TruncatedBody => write!(f, "websocket frame has a truncated body"),
            WebsocketEventError::MissingTrailingCrlf => write!(
                f,
                "websocket frame has a truncated is missing trailing crlf"
            ),
            WebsocketEventError::MissingLength => write!(f, "websocket frame was missing length"),
        }
    }
}

impl Error for WebsocketEventError {}
