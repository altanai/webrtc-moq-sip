use crate::config::SipJsConfig;

/// Renders a SIP.js bootstrap snippet for browser-side signaling.
pub fn render_sip_js_bootstrap(cfg: &SipJsConfig) -> String {
    format!(
        "import {{ UserAgent }} from \"sip.js\";\n\nconst ua = new UserAgent({{\n  uri: \"{}\",\n  authorizationUsername: \"{}\",\n  displayName: \"{}\",\n  transportOptions: {{\n    server: \"{}\"\n  }},\n  register: {}\n}});\n\nawait ua.start();\n",
        cfg.sip_uri,
        cfg.authorization_user,
        cfg.display_name,
        cfg.ws_server,
        cfg.register
    )
}

#[cfg(test)]
mod tests {
    use crate::config::SipJsConfig;

    use super::render_sip_js_bootstrap;

    #[test]
    fn snippet_contains_server_and_uri() {
        let cfg = SipJsConfig::default();
        let snippet = render_sip_js_bootstrap(&cfg);

        assert!(snippet.contains("sip:alice@example.com"));
        assert!(snippet.contains("wss://sip.example.com/ws"));
    }
}
