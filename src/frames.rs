/// Returns a WebSocket-over-HTTP formatted control message
pub fn ws_control(kind: &str) -> Vec<u8> {
    format!("{kind}\r\n").into_bytes()
}

pub fn ws_text(msg: &str) -> Vec<u8> {
    format!("TEXT {:x}\r\n{}\r\n", msg.len(), msg).into_bytes()
}

/// Returns a TEXT-wrapped GRIP subscription command for the given channel
pub fn ws_sub(ch: &str) -> Vec<u8> {
    let control_msg = format!("c:{{\"type\":\"subscribe\",\"channel\":\"{}\"}}", ch);
    ws_text(&control_msg)
}

/// Returns a TEXT-wrapped GRIP unsubscribe command for the given channel
pub fn ws_unsub(ch: &str) -> Vec<u8> {
    let control_msg = format!("c:{{\"type\":\"unsubscribe\",\"channel\":\"{}\"}}", ch);
    ws_text(&control_msg)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ws_control() {
        let frame = ws_control("OPEN");
        assert_eq!(frame, b"OPEN\r\n");

        let frame = ws_control("PING");
        assert_eq!(frame, b"PING\r\n");

        let frame = ws_control("DISCONNECT");
        assert_eq!(frame, b"DISCONNECT\r\n");
    }

    #[test]
    fn test_ws_text() {
        let frame = ws_text("hello");
        assert_eq!(frame, b"TEXT 5\r\nhello\r\n");

        let frame = ws_text("hello world");
        assert_eq!(frame, b"TEXT b\r\nhello world\r\n");

        let frame = ws_text("");
        assert_eq!(frame, b"TEXT 0\r\n\r\n");
    }

    #[test]
    fn test_ws_sub() {
        let frame = ws_sub("test-channel");
        let expected = b"TEXT 2f\r\nc:{\"type\":\"subscribe\",\"channel\":\"test-channel\"}\r\n";
        assert_eq!(frame, expected);

        let frame = ws_sub("my-room");
        let expected = b"TEXT 2a\r\nc:{\"type\":\"subscribe\",\"channel\":\"my-room\"}\r\n";
        assert_eq!(frame, expected);
    }

    #[test]
    fn test_ws_unsub() {
        let frame = ws_unsub("test-channel");
        let expected = b"TEXT 31\r\nc:{\"type\":\"unsubscribe\",\"channel\":\"test-channel\"}\r\n";
        assert_eq!(frame, expected);

        let frame = ws_unsub("my-room");
        let expected = b"TEXT 2c\r\nc:{\"type\":\"unsubscribe\",\"channel\":\"my-room\"}\r\n";
        assert_eq!(frame, expected);
    }
}
