use serde_json::Value;

use tauri_app_lib::{
    config_composer::compose_config,
    models::{NodeProtocol, ParsedNode, ProxyMode},
};

#[test]
fn compose_config_preserves_emoji_in_meta_display_name() {
    let node = ParsedNode {
        id: "n1".to_string(),
        name: "🇭🇰 香港 ① VIP".to_string(),
        protocol: NodeProtocol::Vmess,
        server: "a.example.com".to_string(),
        port: 443,
        uuid: "123e4567-e89b-12d3-a456-426614174000".to_string(),
        security: Some("auto".to_string()),
        network: Some("ws".to_string()),
        tls: Some(true),
        sni: None,
        alpn: None,
    };

    let cfg = compose_config(&[node], ProxyMode::Global, None).expect("compose should work");
    let outbounds = cfg
        .get("outbounds")
        .and_then(Value::as_array)
        .expect("outbounds array expected");

    // Find the vmess outbound (not direct/block/template)
    let vmess = outbounds
        .iter()
        .find(|o| o.get("protocol").and_then(Value::as_str) == Some("vmess"))
        .expect("vmess outbound exists");

    let display_name = vmess
        .get("meta")
        .and_then(|m| m.get("displayName"))
        .and_then(Value::as_str)
        .unwrap_or("");

    assert_eq!(display_name, "🇭🇰 香港 ① VIP");
}
