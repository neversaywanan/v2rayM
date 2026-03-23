use serde_json::Value;

use tauri_app_lib::{
    config_composer::compose_config,
    models::{NodeProtocol, ParsedNode, ProxyMode},
};

fn sample_nodes() -> Vec<ParsedNode> {
    vec![
        ParsedNode {
            id: "n1".to_string(),
            name: "node-1".to_string(),
            protocol: NodeProtocol::Vmess,
            server: "a.example.com".to_string(),
            port: 443,
            uuid: "123e4567-e89b-12d3-a456-426614174000".to_string(),
            security: Some("auto".to_string()),
            network: Some("ws".to_string()),
            tls: Some(true),
            sni: None,
            alpn: None,
        },
        ParsedNode {
            id: "n2".to_string(),
            name: "node-2".to_string(),
            protocol: NodeProtocol::Vless,
            server: "b.example.com".to_string(),
            port: 8443,
            uuid: "123e4567-e89b-12d3-a456-426614174001".to_string(),
            security: Some("none".to_string()),
            network: Some("tcp".to_string()),
            tls: Some(false),
            sni: None,
            alpn: None,
        },
    ]
}

#[test]
fn creates_proxy_outbounds_plus_direct_and_block() {
    let config =
        compose_config(&sample_nodes(), ProxyMode::Global, None).expect("compose should work");
    let outbounds = config
        .get("outbounds")
        .and_then(Value::as_array)
        .expect("outbounds array expected");
    // 2 proxy outbounds + special template node + direct + block
    assert_eq!(outbounds.len(), 5);
    assert!(outbounds
        .iter()
        .any(|x| x.get("tag").and_then(Value::as_str) == Some("direct")));
    assert!(outbounds
        .iter()
        .any(|x| x.get("tag").and_then(Value::as_str) == Some("block")));
}

#[test]
fn routing_mode_changes_final_rule() {
    let direct_cfg = compose_config(&sample_nodes(), ProxyMode::Direct, None)
        .expect("direct compose should work");
    let global_cfg = compose_config(&sample_nodes(), ProxyMode::Global, None)
        .expect("global compose should work");
    let rule_cfg =
        compose_config(&sample_nodes(), ProxyMode::Rule, None).expect("rule compose should work");

    let direct_last = last_outbound_tag(&direct_cfg);
    let global_last = last_outbound_tag(&global_cfg);
    let rule_last = last_outbound_tag(&rule_cfg);

    assert_eq!(direct_last, "direct");
    assert_ne!(global_last, direct_last);
    assert_ne!(rule_last, direct_last);
}

fn last_outbound_tag(config: &Value) -> String {
    config
        .get("routing")
        .and_then(|v| v.get("rules"))
        .and_then(Value::as_array)
        .and_then(|rules| rules.last())
        .and_then(|rule| rule.get("outboundTag"))
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string()
}
