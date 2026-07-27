//! webrtc-moq-sip provides a config-first API to bootstrap WebRTC calls,
//! optional MoQ publication, and SIP.js signaling integration.

pub mod config;
pub mod error;
pub mod interop;
pub mod orchestrator;
pub mod sip_js;
#[cfg(feature = "runtime-adapters")]
pub mod adapters;

pub use config::{CallMode, LibraryConfig, MoqPublishConfig, SipJsConfig, WebRtcConfig};
pub use error::InteropError;
pub use interop::{InteropRuntime, MoqPublisher, WebRtcEngine};
pub use orchestrator::{StartupAction, StartupPlan, build_startup_plan};
pub use sip_js::render_sip_js_bootstrap;

#[cfg(test)]
mod tests {
    use crate::{CallMode, LibraryConfig, MoqPublishConfig, SipJsConfig, WebRtcConfig, build_startup_plan};

    #[test]
    fn builds_plan_for_call_and_moq_publish() {
        let cfg = LibraryConfig {
            mode: CallMode::CallAndPublish,
            webrtc: WebRtcConfig::default(),
            sip_js: Some(SipJsConfig::default()),
            moq: Some(MoqPublishConfig::default()),
        };

        let plan = build_startup_plan(&cfg).expect("plan should be valid");
        assert_eq!(plan.actions.len(), 3);
    }
}
