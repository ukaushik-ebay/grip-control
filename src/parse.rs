use std::error::Error;

use crate::error::WebsocketEventError;

pub enum WebsocketEvent<'a> {
    Open,
    Text(&'a [u8]),
    Binary(&'a [u8]),
    Ping,
    Close,
    Disconnect,
}

impl<'a> WebsocketEvent<'a> {
    pub fn parse_frame(resp_body: &'a [u8]) -> Result<WebsocketEvent<'a>, impl Error> {
        if let Some(header_end) = resp_body.windows(2).position(|w| w == b"\r\n") {
            let header = std::str::from_utf8(&resp_body[..header_end]);

            println!("COMMAND: {:?}", header);
            if let Ok(h) = header {
                match h {
                    "OPEN" => Ok(WebsocketEvent::Open),
                    "TEXT" => Ok(WebsocketEvent::Text(&resp_body[header_end + 2..])),
                    "BINARY" => Ok(WebsocketEvent::Binary(&resp_body[header_end + 2..])),
                    "PING" => Ok(WebsocketEvent::Ping),
                    "CLOSE" => Ok(WebsocketEvent::Close),
                    "DISCONNECT" => Ok(WebsocketEvent::Disconnect),
                    _ => Err(WebsocketEventError::UnrecognizedCommand),
                }
            } else {
                Err(WebsocketEventError::ParseError)
            }
        } else {
            println!("FAILED TO PARSE");
            Err(WebsocketEventError::ParseError)
        }
    }
}
