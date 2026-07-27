use crate::config::{CallMode, LibraryConfig};
use crate::error::InteropError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StartupAction {
    StartWebRtcCall,
    StartMoqPublisher,
    EmitSipJsBootstrap,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartupPlan {
    pub actions: Vec<StartupAction>,
    pub namespace: String,
}

/// Build a deterministic startup plan from user configuration.
pub fn build_startup_plan(cfg: &LibraryConfig) -> Result<StartupPlan, InteropError> {
    validate(cfg)?;

    let namespace = cfg
        .moq
        .as_ref()
        .map(|m| m.namespace.clone())
        .unwrap_or_else(|| format!("webrtc/{}/{}", cfg.webrtc.room, cfg.webrtc.publisher_id));

    let mut actions = Vec::new();

    match cfg.mode {
        CallMode::Call => {
            actions.push(StartupAction::StartWebRtcCall);
            if cfg.sip_js.is_some() {
                actions.push(StartupAction::EmitSipJsBootstrap);
            }
        }
        CallMode::Publish => {
            actions.push(StartupAction::StartMoqPublisher);
        }
        CallMode::CallAndPublish => {
            actions.push(StartupAction::StartWebRtcCall);
            actions.push(StartupAction::StartMoqPublisher);
            actions.push(StartupAction::EmitSipJsBootstrap);
        }
    }

    Ok(StartupPlan { actions, namespace })
}

fn validate(cfg: &LibraryConfig) -> Result<(), InteropError> {
    if !cfg.webrtc.audio && !cfg.webrtc.video {
        return Err(InteropError::MissingMedia);
    }

    if matches!(cfg.mode, CallMode::Publish | CallMode::CallAndPublish) && cfg.moq.is_none() {
        return Err(InteropError::MissingMoqConfig);
    }

    if matches!(cfg.mode, CallMode::CallAndPublish) && cfg.sip_js.is_none() {
        return Err(InteropError::MissingSipJsConfig);
    }

    if let Some(moq) = cfg.moq.as_ref() {
        if moq.keyframe_priority > u8::MAX as u16 {
            return Err(InteropError::InvalidKeyframePriority(moq.keyframe_priority as u16));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::config::{CallMode, LibraryConfig, MoqPublishConfig, SipJsConfig, WebRtcConfig};
    use crate::error::InteropError;

    use super::build_startup_plan;

    #[test]
    fn publish_mode_requires_moq() {
        let cfg = LibraryConfig {
            mode: CallMode::Publish,
            webrtc: WebRtcConfig::default(),
            sip_js: None,
            moq: None,
        };

        let err = build_startup_plan(&cfg).expect_err("plan should fail");
        assert_eq!(err, InteropError::MissingMoqConfig);
    }

    #[test]
    fn call_mode_allows_no_sip() {
        let cfg = LibraryConfig {
            mode: CallMode::Call,
            webrtc: WebRtcConfig::default(),
            sip_js: None,
            moq: None,
        };

        let plan = build_startup_plan(&cfg).expect("plan should build");
        assert_eq!(plan.actions.len(), 1);
    }

    #[test]
    fn call_and_publish_needs_both_sections() {
        let cfg = LibraryConfig {
            mode: CallMode::CallAndPublish,
            webrtc: WebRtcConfig::default(),
            sip_js: Some(SipJsConfig::default()),
            moq: Some(MoqPublishConfig::default()),
        };

        let plan = build_startup_plan(&cfg).expect("plan should build");
        assert_eq!(plan.actions.len(), 3);
    }
}
