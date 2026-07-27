use webrtc_moq_sip::{
    CallMode, LibraryConfig, MoqPublishConfig, SipJsConfig, WebRtcConfig, build_startup_plan,
    render_sip_js_bootstrap,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = LibraryConfig {
        mode: CallMode::CallAndPublish,
        webrtc: WebRtcConfig::default(),
        sip_js: Some(SipJsConfig::default()),
        moq: Some(MoqPublishConfig::default()),
    };

    let plan = build_startup_plan(&cfg)?;
    println!("Startup namespace: {}", plan.namespace);
    println!("Actions: {:?}", plan.actions);

    if let Some(sip_cfg) = cfg.sip_js.as_ref() {
        println!("\nGenerated SIP.js bootstrap:\n");
        println!("{}", render_sip_js_bootstrap(sip_cfg));
    }

    Ok(())
}
