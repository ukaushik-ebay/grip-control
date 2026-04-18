# grip-control

A Rust library for working with the [GRIP (Generic Realtime Intermediary Protocol)](https://pushpin.org/docs/protocols/grip/) WebSocket-over-HTTP wire format, as used by [Fastly Fanout](https://developer.fastly.com/learning/concepts/real-time-messaging/fanout/) and compatible proxies.

## Overview

`grip-control` handles the two directions of GRIP WebSocket communication:

- **Parsing** — decode inbound WebSocket event frames arriving from a GRIP proxy
- **Building** — encode outbound control frames to send back through the proxy

Zero external dependencies. The parser borrows directly from the input bytes — no heap allocation for frame payloads.

## Usage

Add to `Cargo.toml`:

```toml
[dependencies]
grip-control = "0.6"
```

### Parsing inbound frames

```rust
use grip_control::parse::WebsocketEvent;

let body: &[u8] = /* request body from GRIP proxy */;

match WebsocketEvent::parse_frame(body)? {
    WebsocketEvent::Open => { /* new connection */ }
    WebsocketEvent::Text(data) => { /* UTF-8 message */ }
    WebsocketEvent::Binary(data) => { /* binary message */ }
    WebsocketEvent::Ping => { /* keep-alive */ }
    WebsocketEvent::Close(data) => { /* connection closing */ }
    WebsocketEvent::Disconnect => { /* proxy disconnect */ }
}
```

### Building outbound frames

```rust
use grip_control::frames::{ws_sub, ws_unsub, ws_text, ws_control};

// Subscribe the connection to a channel
let frame = ws_sub("my-channel");

// Send a text message
let frame = ws_text("hello");

// Unsubscribe
let frame = ws_unsub("my-channel");

// Raw control frame (e.g. "keep-alive\r\n")
let frame = ws_control("keep-alive");
```

## Wire format

Frames follow the GRIP WebSocket-over-HTTP encoding:

```
COMMAND [HEX_LEN]\r\n[BODY]\r\n
```

Channel subscription/unsubscription frames are JSON-encoded GRIP control messages wrapped in a `TEXT` frame:

```
TEXT <len>\r\nc:{"type":"subscribe","channel":"<ch>"}\r\n
```

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
