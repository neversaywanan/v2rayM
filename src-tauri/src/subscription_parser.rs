use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde_json::Value;
use url::Url;

use crate::models::{NodeProtocol, ParsedNode, SubscriptionParseResult};

const INVALID_SAMPLE_CAP: usize = 5;

pub fn parse_subscription_text(input: &str) -> SubscriptionParseResult {
    let mut nodes = Vec::new();
    let mut invalid_samples = Vec::new();
    let mut invalid_count = 0usize;

    // Try to decode as base64 first (whole subscription might be encoded)
    let decoded_input = if let Ok(decoded) = STANDARD.decode(input.trim()) {
        if let Ok(text) = String::from_utf8(decoded) {
            text
        } else {
            input.to_string()
        }
    } else {
        input.to_string()
    };

    for raw_line in decoded_input.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Try decode line as base64
        let line_to_parse = if let Ok(decoded) = STANDARD.decode(line) {
            if let Ok(text) = String::from_utf8(decoded) {
                text
            } else {
                line.to_string()
            }
        } else {
            line.to_string()
        };

        match parse_line(&line_to_parse) {
            Ok(node) => nodes.push(node),
            Err(e) => {
                eprintln!(
                    "[Parser] Failed to parse: {} - Error: {}",
                    &line_to_parse[..line_to_parse.len().min(50)],
                    e
                );
                invalid_count += 1;
                if invalid_samples.len() < INVALID_SAMPLE_CAP {
                    invalid_samples.push(line.to_string());
                }
            }
        }
    }

    eprintln!(
        "[Parser] Total nodes: {}, Invalid: {}",
        nodes.len(),
        invalid_count
    );
    SubscriptionParseResult {
        nodes,
        invalid_count,
        invalid_samples,
    }
}

fn parse_line(line: &str) -> Result<ParsedNode, String> {
    let line = line.trim();

    if let Some(encoded) = line.strip_prefix("vmess://") {
        return parse_vmess(encoded);
    }
    if let Some(rest) = line.strip_prefix("vless://") {
        return parse_vless(rest);
    }
    if let Some(rest) = line.strip_prefix("trojan://") {
        return parse_trojan(rest);
    }
    if let Some(rest) = line.strip_prefix("ss://") {
        return parse_ss(rest);
    }
    if let Some(rest) = line.strip_prefix("ssr://") {
        return parse_ssr(rest);
    }

    Err(format!(
        "unsupported protocol: {}",
        &line[..line.len().min(20)]
    ))
}

fn parse_vmess(encoded: &str) -> Result<ParsedNode, String> {
    let decoded = STANDARD
        .decode(encoded)
        .map_err(|_| "invalid vmess base64".to_string())?;
    let text = String::from_utf8(decoded).map_err(|_| "invalid utf8".to_string())?;
    let v: Value = serde_json::from_str(&text).map_err(|_| "invalid vmess json".to_string())?;

    let name = v
        .get("ps")
        .and_then(Value::as_str)
        .unwrap_or("vmess")
        .to_string();
    let server = v
        .get("add")
        .and_then(Value::as_str)
        .ok_or_else(|| "missing add".to_string())?
        .to_string();
    let port = v
        .get("port")
        .and_then(|p| {
            if let Some(s) = p.as_str() {
                Some(s.to_string())
            } else if let Some(n) = p.as_u64() {
                Some(n.to_string())
            } else {
                None
            }
        })
        .ok_or_else(|| "missing port".to_string())?
        .parse::<u16>()
        .map_err(|_| "invalid port".to_string())?;
    let uuid = v
        .get("id")
        .and_then(Value::as_str)
        .ok_or_else(|| "missing id".to_string())?
        .to_string();
    let network = v.get("net").and_then(Value::as_str).map(|s| s.to_string());
    let tls = v
        .get("tls")
        .and_then(Value::as_str)
        .map(|s| s.eq_ignore_ascii_case("tls") || s.eq_ignore_ascii_case("reality"));
    let sni = v.get("sni").and_then(Value::as_str).map(|s| s.to_string());

    Ok(ParsedNode {
        id: format!("vmess-{server}-{port}-{uuid}"),
        name,
        protocol: NodeProtocol::Vmess,
        server,
        port,
        uuid,
        security: None,
        network,
        tls,
        sni,
        alpn: None,
    })
}

fn parse_vless(rest: &str) -> Result<ParsedNode, String> {
    let url =
        Url::parse(&format!("vless://{rest}")).map_err(|_| "invalid vless url".to_string())?;

    let uuid = url.username().to_string();
    if uuid.is_empty() {
        return Err("missing uuid".to_string());
    }

    let server = url
        .host_str()
        .ok_or_else(|| "missing host".to_string())?
        .to_string();
    let port = url.port().ok_or_else(|| "missing port".to_string())?;
    let name = url.fragment().unwrap_or("vless").to_string();
    let mut security = None;
    let mut network = None;
    let mut tls = None;
    let mut sni = None;
    let mut alpn = None;

    for (k, v) in url.query_pairs() {
        match k.as_ref() {
            "security" => {
                let sec = v.to_string();
                tls = Some(sec.eq_ignore_ascii_case("tls") || sec.eq_ignore_ascii_case("reality"));
                security = Some(sec);
            }
            "type" => network = Some(v.to_string()),
            "sni" => sni = Some(v.to_string()),
            "alpn" => {
                let values: Vec<String> = v
                    .split(',')
                    .map(str::trim)
                    .filter(|x| !x.is_empty())
                    .map(ToString::to_string)
                    .collect();
                if !values.is_empty() {
                    alpn = Some(values);
                }
            }
            _ => {}
        }
    }

    Ok(ParsedNode {
        id: format!("vless-{server}-{port}-{uuid}"),
        name,
        protocol: NodeProtocol::Vless,
        server,
        port,
        uuid,
        security,
        network,
        tls,
        sni,
        alpn,
    })
}

fn parse_trojan(rest: &str) -> Result<ParsedNode, String> {
    let url =
        Url::parse(&format!("trojan://{rest}")).map_err(|_| "invalid trojan url".to_string())?;

    let password = url.username().to_string();
    if password.is_empty() {
        return Err("missing password".to_string());
    }

    let server = url
        .host_str()
        .ok_or_else(|| "missing host".to_string())?
        .to_string();
    let port = url.port().ok_or_else(|| "missing port".to_string())?;
    let name = url.fragment().unwrap_or("trojan").to_string();
    let mut sni = None;
    let mut network = None;

    for (k, v) in url.query_pairs() {
        match k.as_ref() {
            "sni" => sni = Some(v.to_string()),
            "type" => network = Some(v.to_string()),
            _ => {}
        }
    }

    Ok(ParsedNode {
        id: format!("trojan-{server}-{port}-{password}"),
        name,
        protocol: NodeProtocol::Trojan,
        server,
        port,
        uuid: password,
        security: None,
        network,
        tls: Some(true),
        sni,
        alpn: None,
    })
}

fn parse_ss(rest: &str) -> Result<ParsedNode, String> {
    // ss://base64(method:password)@host:port#name
    // or ss://base64(method:password@host:port)#name
    let url = Url::parse(&format!("ss://{rest}")).map_err(|_| "invalid ss url".to_string())?;

    let server = url
        .host_str()
        .ok_or_else(|| "missing host".to_string())?
        .to_string();
    let port = url.port().ok_or_else(|| "missing port".to_string())?;
    let name = url.fragment().unwrap_or("ss").to_string();

    // Try to decode userinfo
    let userinfo = url.username();
    let method_password = if userinfo.contains(':') {
        userinfo.to_string()
    } else {
        // Try base64 decode
        STANDARD
            .decode(userinfo)
            .ok()
            .and_then(|d| String::from_utf8(d).ok())
            .unwrap_or_else(|| userinfo.to_string())
    };

    Ok(ParsedNode {
        id: format!("ss-{server}-{port}-{method_password}"),
        name,
        protocol: NodeProtocol::Ss,
        server,
        port,
        uuid: method_password,
        security: None,
        network: None,
        tls: Some(false),
        sni: None,
        alpn: None,
    })
}

fn parse_ssr(rest: &str) -> Result<ParsedNode, String> {
    // ssr://base64(server:port:protocol:method:obfs:password_base64/?params)
    let decoded = STANDARD
        .decode(rest)
        .map_err(|_| "invalid ssr base64".to_string())?;
    let text = String::from_utf8(decoded).map_err(|_| "invalid utf8".to_string())?;

    let parts: Vec<&str> = text.split(':').collect();
    if parts.len() < 6 {
        return Err("invalid ssr format".to_string());
    }

    let server = parts[0].to_string();
    let port = parts[1]
        .parse::<u16>()
        .map_err(|_| "invalid port".to_string())?;
    let name = text
        .split('/')
        .nth(1)
        .and_then(|s| s.split('?').nth(1))
        .and_then(|s| s.split('&').find(|p| p.starts_with("remarks=")))
        .map(|s| {
            let remarks = s.trim_start_matches("remarks=");
            STANDARD
                .decode(remarks)
                .ok()
                .and_then(|d| String::from_utf8(d).ok())
                .unwrap_or_else(|| remarks.to_string())
        })
        .unwrap_or_else(|| "ssr".to_string());

    Ok(ParsedNode {
        id: format!("ssr-{server}-{port}-{}", parts.get(5).unwrap_or(&"")),
        name,
        protocol: NodeProtocol::Ssr,
        server,
        port,
        uuid: parts.get(5).unwrap_or(&"").to_string(),
        security: None,
        network: None,
        tls: Some(false),
        sni: None,
        alpn: None,
    })
}
