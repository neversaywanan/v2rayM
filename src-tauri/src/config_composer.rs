use serde_json::{json, Value};

use crate::models::{NodeProtocol, ParsedNode, ProxyMode};

pub fn compose_config(
    nodes: &[ParsedNode],
    mode: ProxyMode,
    base_config_raw: Option<&str>,
) -> Result<Value, String> {
    let mut root = if let Some(raw) = base_config_raw {
        serde_json::from_str::<Value>(raw).map_err(|_| "invalid base config json".to_string())?
    } else {
        json!({})
    };

    let mut outbounds = Vec::new();

    for (idx, node) in nodes.iter().enumerate() {
        outbounds.push(node_to_outbound(node, idx + 1));
    }

    // Add the special template node at the end
    outbounds.push(json!({
        "tag": "_建议每日更新一次订阅以及定期检查设备当前时间是否已准确到分钟",
        "protocol": "vmess",
        "settings": {
            "vnext": [
                {
                    "address": "127.0.0.1",
                    "port": 1088,
                    "users": [
                        {
                            "id": "9eff7b62-3d3c-4c3b-8aca-37767fba1282",
                            "encryption": "auto",
                            "alterId": 0,
                            "security": "auto"
                        }
                    ]
                }
            ]
        },
        "streamSettings": {
            "network": "ws",
            "wsSettings": {
                "headers": {
                    "Host": ""
                },
                "path": ""
            }
        }
    }));

    outbounds.push(json!({
        "tag": "direct",
        "protocol": "freedom",
        "settings": {}
    }));
    outbounds.push(json!({
        "tag": "block",
        "protocol": "blackhole",
        "settings": {}
    }));

    let proxy_default_tag = outbounds
        .iter()
        .find_map(|o| {
            let tag = o.get("tag").and_then(Value::as_str)?;
            let proto = o.get("protocol").and_then(Value::as_str).unwrap_or("");
            // Pick the first real proxy outbound (not freedom/blackhole/the template note)
            if matches!(
                proto,
                "vmess" | "vless" | "trojan" | "shadowsocks" | "shadowsocksr"
            ) && !tag.starts_with('_')
            {
                Some(tag)
            } else {
                None
            }
        })
        .unwrap_or("direct");

    let rules = match mode {
        ProxyMode::Direct => vec![json!({
            "type": "field",
            "network": "tcp,udp",
            "outboundTag": "direct"
        })],
        ProxyMode::Global => vec![json!({
            "type": "field",
            "network": "tcp,udp",
            "outboundTag": proxy_default_tag
        })],
        ProxyMode::Rule => vec![
            json!({
                "type": "field",
                "ip": ["geoip:private"],
                "outboundTag": "direct"
            }),
            json!({
                "type": "field",
                "network": "tcp,udp",
                "outboundTag": proxy_default_tag
            }),
        ],
    };

    root["outbounds"] = Value::Array(outbounds);
    root["routing"] = json!({
        "domainStrategy": "AsIs",
        "rules": rules,
    });

    Ok(root)
}

fn node_to_outbound(node: &ParsedNode, _index: usize) -> Value {
    let tag = sanitize_tag(&node.name);
    let network = node.network.clone().unwrap_or_else(|| "tcp".to_string());
    let security = if node.tls.unwrap_or(false) {
        "tls"
    } else {
        "none"
    };

    // Preserve the original display name (may include emoji) in a custom field.
    // V2Ray ignores unknown fields; our UI can read this back for display.
    let meta = json!({
        "displayName": node.name,
    });

    match &node.protocol {
        NodeProtocol::Vmess => {
            let mut outbound = json!({
                "tag": tag,
                "protocol": "vmess",
                "meta": meta,
                "settings": {
                    "vnext": [{
                        "address": node.server,
                        "port": node.port,
                        "users": [{
                            "id": node.uuid,
                            "security": node.security.clone().unwrap_or_else(|| "auto".to_string()),
                        }]
                    }]
                },
                "streamSettings": {
                    "network": network,
                    "security": security
                }
            });

            // Add WebSocket settings if network is ws
            if network == "ws" {
                if let Some(ref mut stream_settings) = outbound["streamSettings"].as_object_mut() {
                    stream_settings.insert(
                        "wsSettings".to_string(),
                        json!({
                            "headers": {},
                            "path": ""
                        }),
                    );
                }
            }

            outbound
        }
        NodeProtocol::Vless => {
            let mut outbound = json!({
                "tag": tag,
                "protocol": "vless",
                "meta": meta,
                "settings": {
                    "vnext": [{
                        "address": node.server,
                        "port": node.port,
                        "users": [{
                            "id": node.uuid,
                            "encryption": node.security.clone().unwrap_or_else(|| "none".to_string()),
                        }]
                    }]
                },
                "streamSettings": {
                    "network": network,
                    "security": security
                }
            });

            // Add WebSocket settings if network is ws
            if network == "ws" {
                if let Some(ref mut stream_settings) = outbound["streamSettings"].as_object_mut() {
                    stream_settings.insert(
                        "wsSettings".to_string(),
                        json!({
                            "headers": {},
                            "path": ""
                        }),
                    );
                }
            }

            outbound
        }
        NodeProtocol::Trojan => json!({
            "tag": tag,
            "protocol": "trojan",
            "meta": meta,
            "settings": {
                "servers": [{
                    "address": node.server,
                    "port": node.port,
                    "password": node.uuid,
                }]
            },
            "streamSettings": {
                "network": network,
                "security": "tls"
            }
        }),
        NodeProtocol::Ss => {
            let parts: Vec<&str> = node.uuid.split(':').collect();
            let (method, password) = if parts.len() >= 2 {
                (parts[0].to_string(), parts[1].to_string())
            } else {
                ("aes-256-gcm".to_string(), node.uuid.clone())
            };
            json!({
                "tag": tag,
                "protocol": "shadowsocks",
                "meta": meta,
                "settings": {
                    "servers": [{
                        "address": node.server,
                        "port": node.port,
                        "method": method,
                        "password": password,
                    }]
                }
            })
        }
        NodeProtocol::Ssr => json!({
            "tag": tag,
            "protocol": "shadowsocksr",
            "meta": meta,
            "settings": {
                "servers": [{
                    "address": node.server,
                    "port": node.port,
                    "method": "aes-256-cfb",
                    "password": node.uuid,
                }]
            }
        }),
    }
}

fn protocol_name(protocol: &NodeProtocol) -> &'static str {
    match protocol {
        NodeProtocol::Vmess => "vmess",
        NodeProtocol::Vless => "vless",
        NodeProtocol::Trojan => "trojan",
        NodeProtocol::Ss => "ss",
        NodeProtocol::Ssr => "ssr",
    }
}

/// Convert a node display name into a valid V2Ray tag.
/// V2Ray tags must not contain spaces or certain special chars.
/// Strategy: replace non-ASCII-word chars with "_", trim edges, deduplicate "_".
fn sanitize_tag(name: &str) -> String {
    // Keep ASCII letters, digits, Chinese/CJK chars and common safe symbols
    let raw: String = name
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '.' {
                c
            } else {
                '_'
            }
        })
        .collect();

    // Collapse runs of underscores and trim edges
    let mut tag = String::new();
    let mut prev_under = false;
    for c in raw.chars() {
        if c == '_' {
            if !prev_under && !tag.is_empty() {
                tag.push('_');
            }
            prev_under = true;
        } else {
            tag.push(c);
            prev_under = false;
        }
    }
    let tag = tag.trim_matches('_').to_string();

    // Fall back to a safe default if the result would be empty
    if tag.is_empty() {
        return "proxy_node".to_string();
    }
    tag
}
