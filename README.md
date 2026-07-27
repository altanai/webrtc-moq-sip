# webrtc-moq-sip

Config-first Rust library to bootstrap:

- WebRTC call sessions (powered by `webrtc-rs` integration points),
- Media over QUIC publish flows,
- SIP.js browser signaling setup.

This crate is designed for publication on crates.io and for use as a reusable SDK in projects that need WebRTC + MoQ interop.

## What It Provides

- Typed config model with `serde` support
- Startup planner for `call`, `publish`, or `call_and_publish`
- SIP.js bootstrap generator for browser-side SIP over WebSocket
- Sensible defaults similar to quick-start JS SDK configuration styles

## Install

```toml
[dependencies]
webrtc-moq-sip = "0.1"
```

## Quick Start

```rust
use webrtc_moq_sip::{
    build_startup_plan, render_sip_js_bootstrap, CallMode, LibraryConfig, MoqPublishConfig,
    SipJsConfig, WebRtcConfig,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = LibraryConfig {
        mode: CallMode::CallAndPublish,
        webrtc: WebRtcConfig::default(),
        sip_js: Some(SipJsConfig::default()),
        moq: Some(MoqPublishConfig::default()),
    };

    let plan = build_startup_plan(&cfg)?;
    println!("startup namespace: {}", plan.namespace);
    println!("actions: {:?}", plan.actions);

    if let Some(sip) = cfg.sip_js.as_ref() {
        let js = render_sip_js_bootstrap(sip);
        println!("SIP.js bootstrap script:\n{}", js);
    }

    Ok(())
}
```

## JSON Config Example

```json
{
  "mode": "call_and_publish",
  "webrtc": {
    "room": "interop-lab",
    "publisher_id": "alice",
    "stun_urls": ["stun:stun.l.google.com:19302"],
    "audio": true,
    "video": true
  },
  "sip_js": {
    "ws_server": "wss://sip.example.com/ws",
    "sip_uri": "sip:alice@example.com",
    "authorization_user": "alice",
    "display_name": "Alice",
    "register": true
  },
  "moq": {
    "relay_url": "https://relay.example.com/moq",
    "namespace": "webrtc/interop-lab/alice",
    "video_track": "video/main",
    "audio_track": "audio/main",
    "keyframe_priority": 250
  }
}
```

## Notes

- This crate currently focuses on orchestration and integration contracts.
- Runtime media transport wiring to `webrtc-rs` and concrete MoQ publishers can be plugged in using the generated startup plan.
