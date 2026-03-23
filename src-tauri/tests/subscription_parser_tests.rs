use tauri_app_lib::subscription_parser::parse_subscription_text;

#[test]
fn parses_vmess_line() {
    let vmess_json = r#"{"v":"2","ps":"node-a","add":"example.com","port":"443","id":"123e4567-e89b-12d3-a456-426614174000","aid":"0","net":"ws","type":"none","host":"","path":"/","tls":"tls"}"#;
    let vmess_b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, vmess_json);
    let input = format!("vmess://{vmess_b64}");
    let result = parse_subscription_text(&input);

    assert_eq!(result.nodes.len(), 1);
    assert_eq!(result.nodes[0].name, "node-a");
    assert_eq!(result.invalid_count, 0);
}

#[test]
fn parses_vless_line() {
    let input = "vless://123e4567-e89b-12d3-a456-426614174000@v.example.com:8443?encryption=none&security=tls&type=ws#node-b";
    let result = parse_subscription_text(input);

    assert_eq!(result.nodes.len(), 1);
    assert_eq!(result.nodes[0].name, "node-b");
    assert_eq!(result.nodes[0].server, "v.example.com");
    assert_eq!(result.invalid_count, 0);
}

#[test]
fn keeps_valid_nodes_when_partial_failures_exist() {
    let vmess_json = r#"{"v":"2","ps":"node-a","add":"example.com","port":"443","id":"123e4567-e89b-12d3-a456-426614174000"}"#;
    let vmess_b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, vmess_json);
    let input = format!("vmess://{vmess_b64}\ninvalid://whatever\n#comment\n\n");
    let result = parse_subscription_text(&input);

    assert_eq!(result.nodes.len(), 1);
    assert_eq!(result.invalid_count, 1);
    assert_eq!(result.invalid_samples.len(), 1);
}

#[test]
fn ignores_blank_and_comment_lines() {
    let input = "\n# comment\n\n";
    let result = parse_subscription_text(input);

    assert!(result.nodes.is_empty());
    assert_eq!(result.invalid_count, 0);
}

#[test]
fn caps_invalid_samples_at_five() {
    let mut input = String::new();
    for i in 0..8 {
        input.push_str(&format!("bad://{i}\n"));
    }
    let result = parse_subscription_text(&input);

    assert_eq!(result.invalid_count, 8);
    assert_eq!(result.invalid_samples.len(), 5);
}
