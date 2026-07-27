use crate::config::LibraryConfig;
use crate::error::InteropError;
use crate::interop::{MoqPublisher, WebRtcEngine};

/// Minimal WebRTC engine wrapper backed by webrtc-rs types.
pub struct WebRtcRsEngine {
    builder: webrtc::api::APIBuilder,
}

impl Default for WebRtcRsEngine {
    fn default() -> Self {
        Self {
            builder: webrtc::api::APIBuilder::new(),
        }
    }
}

impl WebRtcEngine for WebRtcRsEngine {
    fn start_call(&mut self, _cfg: &LibraryConfig) -> Result<(), InteropError> {
        let _rtc_config = webrtc::peer_connection::configuration::RTCConfiguration::default();
        let _api = self.builder.build();
        Ok(())
    }
}

/// Minimal MoQ publisher wrapper backed by moq-native types.
#[derive(Default)]
pub struct MoqNativePublisher;

impl MoqPublisher for MoqNativePublisher {
    fn start_publish(&mut self, _cfg: &LibraryConfig) -> Result<(), InteropError> {
        let _origin = moq_native::moq_lite::Origin::produce();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{CallMode, LibraryConfig, MoqPublishConfig, SipJsConfig, WebRtcConfig};
    use crate::interop::InteropRuntime;

    use super::{MoqNativePublisher, WebRtcRsEngine};

    #[test]
    fn adapter_runtime_starts() {
        let mut runtime = InteropRuntime {
            webrtc: WebRtcRsEngine::default(),
            moq: MoqNativePublisher,
        };

        let cfg = LibraryConfig {
            mode: CallMode::CallAndPublish,
            webrtc: WebRtcConfig::default(),
            sip_js: Some(SipJsConfig::default()),
            moq: Some(MoqPublishConfig::default()),
        };

        let plan = runtime.start(&cfg).expect("runtime should start with adapters");
        assert_eq!(plan.actions.len(), 3);
    }
}
