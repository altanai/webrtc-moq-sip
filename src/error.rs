use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum InteropError {
    #[error("audio and video are both disabled in WebRTC config")]
    MissingMedia,
    #[error("mode requires MoQ config, but no moq section was provided")]
    MissingMoqConfig,
    #[error("mode requires SIP.js config, but no sip_js section was provided")]
    MissingSipJsConfig,
    #[error("invalid keyframe_priority {0}; expected 0-255")]
    InvalidKeyframePriority(u16),
}
