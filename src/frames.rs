use serde_json::json;

pub fn ws_control(kind: &str) -> Vec<u8> {
    format!("{kind}\r\n").into_bytes()
}

/// Returns a WebSocket-over-HTTP formatted TEXT message
pub fn ws_text(msg: &str) -> Vec<u8> {
    format!("TEXT {:x}\r\n{}\r\n", msg.len(), msg).into_bytes()
}

/// Returns a TEXT-wrapped GRIP subscription command for the given channel
pub fn ws_sub(ch: &str) -> Vec<u8> {
    let payload = json!({
        "type": "subscribe",
        "channel": ch
    });
    let control_msg = format!("c:{payload}");
    ws_text(&control_msg)
}
