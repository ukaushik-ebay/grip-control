use crate::error::WebsocketEventError;

pub enum WebsocketEvent<'a> {
    Open,
    Text(&'a [u8]),
    Binary(&'a [u8]),
    Ping,
    Close(&'a [u8]),
    Disconnect,
}

impl<'a> WebsocketEvent<'a> {
    pub fn parse_frame(resp_body: &'a [u8]) -> Result<WebsocketEvent<'a>, WebsocketEventError> {
        let header_end = resp_body
            .windows(2)
            .position(|w| w == b"\r\n")
            .ok_or(WebsocketEventError::ParseError)?;

        let header = std::str::from_utf8(&resp_body[..header_end])
            .map_err(|_| WebsocketEventError::InvalidUtf8)?;

        let mut parts = header.split(' ');
        let command = parts.next().ok_or(WebsocketEventError::ParseError)?;

        match command {
            "OPEN" => Ok(WebsocketEvent::Open),
            "PING" => Ok(WebsocketEvent::Ping),
            "DISCONNECT" => Ok(WebsocketEvent::Disconnect),
            "TEXT" | "BINARY" | "CLOSE" => {
                let len_hex = parts.next().ok_or(WebsocketEventError::MissingLength)?;
                let len = usize::from_str_radix(len_hex, 16)
                    .map_err(|_| WebsocketEventError::InvalidLength)?;

                let body_start = header_end + 2;
                let body_end = body_start + len;

                if resp_body.len() < body_end + 2 {
                    return Err(WebsocketEventError::TruncatedBody);
                }

                let body = &resp_body[body_start..body_end];

                if &resp_body[body_end..body_end + 2] != b"\r\n" {
                    return Err(WebsocketEventError::MissingTrailingCrlf);
                }

                match command {
                    "TEXT" => Ok(WebsocketEvent::Text(body)),
                    "BINARY" => Ok(WebsocketEvent::Binary(body)),
                    "CLOSE" => Ok(WebsocketEvent::Close(body)),
                    _ => unreachable!(),
                }
            }

            _ => Err(WebsocketEventError::UnrecognizedCommand),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_open() {
        let frame: &[u8; 6] = b"OPEN\r\n";
        let event = WebsocketEvent::parse_frame(frame).expect("Parsing OPEN frame failed.");
        assert!(matches!(event, WebsocketEvent::Open));
    }

    #[test]
    fn parse_ping() {
        let frame = b"PING\r\n";
        let event = WebsocketEvent::parse_frame(frame).expect("Parsing PING frame failed.");
        assert!(matches!(event, WebsocketEvent::Ping));
    }

    #[test]
    fn parse_disconnect() {
        let frame = b"DISCONNECT\r\n";
        let event = WebsocketEvent::parse_frame(frame).expect("Parsing DISCONNECT frame failed.");
        assert!(matches!(event, WebsocketEvent::Disconnect));
    }

    #[test]
    fn parse_close() {
        let frame = b"CLOSE 2\r\n\x03\xe8\r\n";
        let event = WebsocketEvent::parse_frame(frame).expect("Parsing CLOSE frame failed.");
        assert!(matches!(event, WebsocketEvent::Close(_)));
    }

    #[test]
    fn parse_text() {
        let frame = b"TEXT 5\r\nhello\r\n";
        let event = WebsocketEvent::parse_frame(frame).expect("Parsing TEXT frame failed.");
        assert!(matches!(event, WebsocketEvent::Text(b"hello")));
    }

    #[test]
    fn parse_text_2() {
        let frame = b"TEXT B\r\nhello world\r\n";
        let event = WebsocketEvent::parse_frame(frame).expect("Parsing TEXT frame failed.");
        assert!(matches!(event, WebsocketEvent::Text(b"hello world")));
    }

    #[test]
    fn parse_text_3() {
        let frame = b"TEXT 1C\r\nhere is another nice message\r\n";
        let event = WebsocketEvent::parse_frame(frame).expect("Parsing TEXT frame failed.");
        assert!(matches!(
            event,
            WebsocketEvent::Text(b"here is another nice message")
        ));
    }
}
