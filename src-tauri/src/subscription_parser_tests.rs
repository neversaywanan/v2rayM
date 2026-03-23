#[cfg(test)]
mod tests {
    use crate::subscription_parser::parse_subscription_text;
    use base64::Engine;

    fn vmess_line(ps: &str, add: &str, port: u16, id: &str) -> String {
        let json = format!(
            r#"{{"ps":"{}","add":"{}","port":{},"id":"{}","aid":0,"net":"ws","type":"none","host":"","path":"/","tls":""}}"#,
            ps, add, port, id
        );
        let b64 = base64::engine::general_purpose::STANDARD.encode(json.as_bytes());
        format!("vmess://{}", b64)
    }

    #[test]
    fn vmess_nodes_with_same_host_port_but_different_uuid_have_unique_ids() {
        let a = vmess_line(
            "n1",
            "example.com",
            443,
            "11111111-1111-1111-1111-111111111111",
        );
        let b = vmess_line(
            "n2",
            "example.com",
            443,
            "22222222-2222-2222-2222-222222222222",
        );
        let input = format!("{}\n{}\n", a, b);

        let res = parse_subscription_text(&input);
        assert_eq!(res.nodes.len(), 2);
        assert_ne!(res.nodes[0].id, res.nodes[1].id);
    }
}
