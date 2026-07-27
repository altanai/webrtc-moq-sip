use serde::{Deserialize, Serialize};
use url::Url;

/// High-level startup behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CallMode {
    /// Start only the WebRTC call path.
    Call,
    /// Start only the MoQ publish path.
    Publish,
    /// Start call and publish simultaneously.
    CallAndPublish,
}

/// Top-level configuration for the library.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LibraryConfig {
    pub mode: CallMode,
    pub webrtc: WebRtcConfig,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sip_js: Option<SipJsConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub moq: Option<MoqPublishConfig>,
}

impl Default for LibraryConfig {
    fn default() -> Self {
        Self {
            mode: CallMode::Call,
            webrtc: WebRtcConfig::default(),
            sip_js: Some(SipJsConfig::default()),
            moq: None,
        }
    }
}

/// Core WebRTC settings that map to a peer connection and media capture profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebRtcConfig {
    pub room: String,
    pub publisher_id: String,
    pub stun_urls: Vec<Url>,
    pub audio: bool,
    pub video: bool,
}

impl Default for WebRtcConfig {
    fn default() -> Self {
        Self {
            room: "demo-room".to_owned(),
            publisher_id: "publisher-1".to_owned(),
            stun_urls: vec![Url::parse("stun:stun.l.google.com:19302").expect("valid default STUN URL")],
            audio: true,
            video: true,
        }
    }
}

/// SIP.js signaling profile used by browser clients.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SipJsConfig {
    pub ws_server: Url,
    pub sip_uri: String,
    pub authorization_user: String,
    pub display_name: String,
    #[serde(default)]
    pub register: bool,
}

impl Default for SipJsConfig {
    fn default() -> Self {
        Self {
            ws_server: Url::parse("wss://sip.example.com/ws").expect("valid default SIP WS URL"),
            sip_uri: "sip:alice@example.com".to_owned(),
            authorization_user: "alice".to_owned(),
            display_name: "Alice".to_owned(),
            register: true,
        }
    }
}

/// Media over QUIC publishing settings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MoqPublishConfig {
    pub relay_url: Url,
    pub namespace: String,
    pub video_track: String,
    pub audio_track: String,
    #[serde(default)]
    pub keyframe_priority: u16,
}

impl Default for MoqPublishConfig {
    fn default() -> Self {
        Self {
            relay_url: Url::parse("https://relay.example.com/moq").expect("valid default relay URL"),
            namespace: "webrtc/demo-room/publisher-1".to_owned(),
            video_track: "video/main".to_owned(),
            audio_track: "audio/main".to_owned(),
            keyframe_priority: 250,
        }
    }
}
