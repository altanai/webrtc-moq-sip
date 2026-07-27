use crate::config::LibraryConfig;
use crate::error::InteropError;
use crate::orchestrator::{StartupAction, StartupPlan, build_startup_plan};
use crate::sip_js::render_sip_js_bootstrap;

/// Adapter for a concrete WebRTC implementation such as webrtc-rs.
pub trait WebRtcEngine {
    fn start_call(&mut self, cfg: &LibraryConfig) -> Result<(), InteropError>;
}

/// Adapter for a concrete MoQ publisher implementation.
pub trait MoqPublisher {
    fn start_publish(&mut self, cfg: &LibraryConfig) -> Result<(), InteropError>;
}

/// Runtime orchestration helper that executes startup actions in order.
pub struct InteropRuntime<W, M>
where
    W: WebRtcEngine,
    M: MoqPublisher,
{
    pub webrtc: W,
    pub moq: M,
}

impl<W, M> InteropRuntime<W, M>
where
    W: WebRtcEngine,
    M: MoqPublisher,
{
    pub fn start(&mut self, cfg: &LibraryConfig) -> Result<StartupPlan, InteropError> {
        let plan = build_startup_plan(cfg)?;

        for action in &plan.actions {
            match action {
                StartupAction::StartWebRtcCall => self.webrtc.start_call(cfg)?,
                StartupAction::StartMoqPublisher => self.moq.start_publish(cfg)?,
                StartupAction::EmitSipJsBootstrap => {
                    if let Some(sip) = cfg.sip_js.as_ref() {
                        let _ = render_sip_js_bootstrap(sip);
                    }
                }
            }
        }

        Ok(plan)
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{CallMode, LibraryConfig, MoqPublishConfig, SipJsConfig, WebRtcConfig};

    use super::{InteropRuntime, MoqPublisher, WebRtcEngine};

    #[derive(Default)]
    struct MockWebRtc {
        pub call_started: bool,
    }

    impl WebRtcEngine for MockWebRtc {
        fn start_call(&mut self, _cfg: &LibraryConfig) -> Result<(), crate::InteropError> {
            self.call_started = true;
            Ok(())
        }
    }

    #[derive(Default)]
    struct MockMoq {
        pub publish_started: bool,
    }

    impl MoqPublisher for MockMoq {
        fn start_publish(&mut self, _cfg: &LibraryConfig) -> Result<(), crate::InteropError> {
            self.publish_started = true;
            Ok(())
        }
    }

    #[test]
    fn runtime_starts_both_paths() {
        let cfg = LibraryConfig {
            mode: CallMode::CallAndPublish,
            webrtc: WebRtcConfig::default(),
            sip_js: Some(SipJsConfig::default()),
            moq: Some(MoqPublishConfig::default()),
        };

        let mut runtime = InteropRuntime {
            webrtc: MockWebRtc::default(),
            moq: MockMoq::default(),
        };

        let plan = runtime.start(&cfg).expect("runtime should start");
        assert_eq!(plan.actions.len(), 3);
        assert!(runtime.webrtc.call_started);
        assert!(runtime.moq.publish_started);
    }
}
