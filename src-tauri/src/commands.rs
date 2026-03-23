use chrono::Utc;
use serde_json::Value;

use crate::config_composer::compose_config;
use crate::errors::{AppError, AppErrorCode};
use crate::models::{
    ApplyPayload, ApplyResult, ConnectPayload, FetchResult, InboundLite, OutboundLite,
    SubscriptionParseResult, SwitchOutboundPayload, UpdatePortPayload, V2RayConfigLite,
};
use crate::ssh_client::SshClient;
use crate::subscription_parser::parse_subscription_text;

pub const DEFAULT_CONFIG_PATH: &str = "/etc/v2ray/config.json";

pub fn resolve_config_path(path: Option<&str>) -> String {
    let v = path.unwrap_or("").trim();
    if v.is_empty() {
        DEFAULT_CONFIG_PATH.to_string()
    } else {
        v.to_string()
    }
}

/// Wraps a closure that performs blocking SSH/IO work in a dedicated
/// OS thread so the Tauri async runtime is never stalled.
async fn run_blocking<F, T>(f: F) -> Result<T, AppError>
where
    F: FnOnce() -> Result<T, AppError> + Send + 'static,
    T: Send + 'static,
{
    tokio::task::spawn_blocking(f)
        .await
        .map_err(|e| AppError::new(AppErrorCode::SshConnectFailed, e.to_string()))?
}

#[tauri::command]
pub async fn connect_and_fetch_config(payload: ConnectPayload) -> Result<FetchResult, AppError> {
    run_blocking(move || {
        let target = resolve_config_path(payload.config_path.as_deref());
        let client = SshClient::connect(&payload)?;
        let raw = client.read_file(&target)?;
        let parsed = parse_config_lite(&raw).ok();
        Ok(FetchResult {
            raw_text: raw,
            parsed,
            used_path: target,
            fetched_at: Utc::now().to_rfc3339(),
        })
    })
    .await
}

#[tauri::command]
pub async fn fetch_subscription(url: String) -> Result<SubscriptionParseResult, AppError> {
    run_blocking(move || {
        let text = reqwest::blocking::get(&url)
            .map_err(|e| AppError::new(AppErrorCode::SubscriptionFetchFailed, e.to_string()))?
            .text()
            .map_err(|e| AppError::new(AppErrorCode::SubscriptionFetchFailed, e.to_string()))?;
        Ok(parse_subscription_text(&text))
    })
    .await
}

#[tauri::command]
pub async fn apply_remote_config(payload: ApplyPayload) -> Result<ApplyResult, AppError> {
    run_blocking(move || {
        let target_path = if payload.target_path.trim().is_empty() {
            resolve_config_path(payload.ssh.config_path.as_deref())
        } else {
            payload.target_path.clone()
        };

        let client = SshClient::connect(&payload.ssh)?;
        let old_raw = client.read_file(&target_path)?;

        let backup_path = format!("{}.bak.{}", target_path, Utc::now().timestamp());
        client
            .write_file_atomic(&backup_path, &old_raw)
            .map_err(|e| AppError::new(AppErrorCode::RemoteUploadFailed, e.message))?;

        let composed = compose_config(
            &payload.selected_nodes,
            payload.mode,
            payload.base_config_raw.as_deref(),
        )
        .map_err(|e| AppError::new(AppErrorCode::JsonParseFailed, e))?;

        let new_text = serde_json::to_string_pretty(&composed)
            .map_err(|e| AppError::new(AppErrorCode::JsonParseFailed, e.to_string()))?;
        client.write_file_atomic(&target_path, &new_text)?;

        let restart_out = client
            .exec("systemctl restart v2ray")
            .map_err(|e| AppError::new(AppErrorCode::RemoteRestartFailed, e.message.clone()))?;
        let status_out = client
            .exec("systemctl status v2ray --no-pager")
            .map_err(|e| AppError::new(AppErrorCode::RemoteRestartFailed, e.message.clone()))?;

        Ok(ApplyResult {
            backup_path,
            used_path: target_path,
            restart_ok: true,
            status_summary: format!("{}\n{}", restart_out, status_out),
            applied_at: Utc::now().to_rfc3339(),
        })
    })
    .await
}

#[tauri::command]
pub async fn rollback_remote_config(payload: ConnectPayload) -> Result<ApplyResult, AppError> {
    run_blocking(move || {
        let target_path = resolve_config_path(payload.config_path.as_deref());
        let client = SshClient::connect(&payload)?;
        let list = client
            .exec(&format!(
                "ls -1t '{}'.bak.* | head -n 1",
                target_path.replace('\'', "'\\''")
            ))
            .map_err(|e| AppError::new(AppErrorCode::RemoteRollbackFailed, e.message))?;
        let backup = list.trim().to_string();
        if backup.is_empty() {
            return Err(AppError::new(
                AppErrorCode::RemoteRollbackFailed,
                "no backup found",
            ));
        }

        client
            .exec(&format!(
                "cp '{}' '{}'",
                backup.replace('\'', "'\\''"),
                target_path.replace('\'', "'\\''")
            ))
            .map_err(|e| AppError::new(AppErrorCode::RemoteRollbackFailed, e.message.clone()))?;

        let restart_out = client
            .exec("systemctl restart v2ray")
            .map_err(|e| AppError::new(AppErrorCode::RemoteRollbackFailed, e.message.clone()))?;
        let status_out = client
            .exec("systemctl status v2ray --no-pager")
            .map_err(|e| AppError::new(AppErrorCode::RemoteRollbackFailed, e.message.clone()))?;

        Ok(ApplyResult {
            backup_path: backup,
            used_path: target_path,
            restart_ok: true,
            status_summary: format!("{}\n{}", restart_out, status_out),
            applied_at: Utc::now().to_rfc3339(),
        })
    })
    .await
}

#[tauri::command]
pub async fn test_proxy_connection(payload: ConnectPayload) -> Result<String, AppError> {
    run_blocking(move || {
        let client = SshClient::connect(&payload)?;

        let check_port_cmd = "netstat -tuln | grep ':1080 ' | wc -l";
        let port_check_result = client
            .exec(check_port_cmd)
            .map_err(|e| AppError::new(AppErrorCode::RemoteCommandFailed, e.message.clone()))?;

        let port_count: i32 = port_check_result.trim().parse().unwrap_or(0);

        if port_count == 0 {
            return Ok("Error: SOCKS5 proxy is not listening on port 1080. Please check if V2Ray service is running.".to_string());
        }

        let direct_test_cmd = "curl -s -o /dev/null -w 'Direct connection HTTP Status: %{http_code}\\nTime Total: %{time_total}s\\n' --connect-timeout 5 --max-time 10 http://www.google.com";
        let direct_result = client
            .exec(direct_test_cmd)
            .map_err(|e| AppError::new(AppErrorCode::RemoteCommandFailed, e.message.clone()))
            .unwrap_or_else(|e| format!("Direct test failed: {}", e.message));

        let proxy_test_cmd = "curl -x socks5://127.0.0.1:1080 -s -o /dev/null -w 'SOCKS5 proxy HTTP Status: %{http_code}\\nTime Total: %{time_total}s\\n' --connect-timeout 10 --max-time 10 http://www.google.com";
        let proxy_result = client
            .exec(proxy_test_cmd)
            .map_err(|e| AppError::new(AppErrorCode::RemoteCommandFailed, e.message.clone()))
            .unwrap_or_else(|e| format!("Proxy test failed: {}", e.message));

        let trace_cmd = "curl -x socks5://127.0.0.1:1080 -s --connect-timeout 10 --max-time 10 https://cloudflare.com/cdn-cgi/trace";
        let trace_result = client
            .exec(trace_cmd)
            .map_err(|e| AppError::new(AppErrorCode::RemoteCommandFailed, e.message.clone()))
            .unwrap_or_else(|_| "ip=unknown\nloc=unknown".to_string());

        let service_status_cmd = "systemctl is-active v2ray";
        let service_status = client
            .exec(service_status_cmd)
            .map_err(|e| AppError::new(AppErrorCode::RemoteCommandFailed, e.message.clone()))
            .unwrap_or_else(|_| "unknown".to_string());

        Ok(format!(
            "Service Status: {}\nPort 1080 Listening: {}\n{}\n{}\n{}",
            service_status.trim(),
            if port_count > 0 { "Yes" } else { "No" },
            direct_result.trim(),
            proxy_result.trim(),
            trace_result.trim()
        ))
    })
    .await
}

fn parse_config_lite(raw: &str) -> Result<V2RayConfigLite, AppError> {
    let value: Value = serde_json::from_str(raw)
        .map_err(|e| AppError::new(AppErrorCode::JsonParseFailed, e.to_string()))?;

    let inbounds = value
        .get("inbounds")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .map(|item| InboundLite {
                    port: item.get("port").and_then(Value::as_u64).map(|x| x as u16),
                    protocol: item
                        .get("protocol")
                        .and_then(Value::as_str)
                        .map(ToString::to_string),
                    listen: item
                        .get("listen")
                        .and_then(Value::as_str)
                        .map(ToString::to_string),
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let outbounds = value
        .get("outbounds")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .map(|item| {
                    let protocol = item.get("protocol").and_then(Value::as_str).map(String::from);
                    let tag = item.get("tag").and_then(Value::as_str).map(String::from);
                    let display_name = item
                        .get("meta")
                        .and_then(|m| m.get("displayName"))
                        .and_then(Value::as_str)
                        .map(String::from);

                    let mut server = None;
                    let mut port = None;

                    if let Some(settings) = item.get("settings") {
                        if let Some(vnext) = settings.get("vnext").and_then(Value::as_array).and_then(|a| a.first()) {
                            server = vnext.get("address").and_then(Value::as_str).map(String::from);
                            port = vnext.get("port").and_then(Value::as_u64).map(|p| p as u16);
                        } else if let Some(servers) = settings.get("servers").and_then(Value::as_array).and_then(|a| a.first()) {
                            server = servers.get("address").and_then(Value::as_str).map(String::from);
                            port = servers.get("port").and_then(Value::as_u64).map(|p| p as u16);
                        }
                    }

                    OutboundLite {
                        protocol,
                        tag,
                        display_name,
                        server,
                        port,
                    }
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let routing_rules_count = value
        .get("routing")
        .and_then(|x| x.get("rules"))
        .and_then(Value::as_array)
        .map(Vec::len)
        .unwrap_or(0);

    // Find the actively-used proxy outbound tag from routing rules.
    // We look for the last rule that doesn't point to "direct" or "block".
    let proxy_protocols = ["vmess", "vless", "trojan", "shadowsocks", "shadowsocksr"];
    let active_outbound_tag = value
        .get("routing")
        .and_then(|r| r.get("rules"))
        .and_then(Value::as_array)
        .and_then(|rules| {
            rules.iter().rev().find_map(|rule| {
                let tag = rule.get("outboundTag").and_then(Value::as_str)?;
                if tag == "direct" || tag == "block" {
                    return None;
                }
                // Confirm this tag corresponds to a real proxy outbound
                let is_proxy = outbounds.iter().any(|o| {
                    o.tag.as_deref() == Some(tag)
                        && o.protocol.as_deref().map(|p| proxy_protocols.contains(&p)).unwrap_or(false)
                });
                if is_proxy { Some(tag.to_string()) } else { None }
            })
        });

    Ok(V2RayConfigLite {
        inbounds,
        outbounds,
        routing_rules_count,
        active_outbound_tag,
    })
}

/// Update only the routing rules in the remote config to point to a different outbound tag,
/// then restart v2ray. Does NOT rewrite the outbounds list.
#[tauri::command]
pub async fn switch_active_outbound(payload: SwitchOutboundPayload) -> Result<(), AppError> {
    run_blocking(move || {
        let target = resolve_config_path(payload.ssh.config_path.as_deref());
        let client = SshClient::connect(&payload.ssh)?;
        let raw = client.read_file(&target)?;

        let mut config: Value = serde_json::from_str(&raw)
            .map_err(|e| AppError::new(AppErrorCode::JsonParseFailed, e.to_string()))?;

        // Rewrite every routing rule that previously pointed to a proxy (not direct/block)
        // so it now points to the newly-selected tag.
        if let Some(rules) = config
            .get_mut("routing")
            .and_then(|r| r.get_mut("rules"))
            .and_then(Value::as_array_mut)
        {
            for rule in rules.iter_mut() {
                if let Some(tag) = rule.get("outboundTag").and_then(Value::as_str) {
                    if tag != "direct" && tag != "block" {
                        rule["outboundTag"] = Value::String(payload.tag.clone());
                    }
                }
            }
        }

        if let Some(routing) = config.get_mut("routing").and_then(Value::as_object_mut) {
            routing.insert(
                "defaultOutboundTag".to_string(),
                Value::String(payload.tag.clone()),
            );
        }

        let new_text = serde_json::to_string_pretty(&config)
            .map_err(|e| AppError::new(AppErrorCode::JsonParseFailed, e.to_string()))?;
        client.write_file_atomic(&target, &new_text)?;
        let _ = client.exec("systemctl restart v2ray && for i in 1 2 3 4 5 6 7 8 9 10; do systemctl is-active v2ray | grep -q active && break; sleep 0.3; done");
        Ok(())
    })
    .await
}

#[tauri::command]
pub async fn update_inbound_port(payload: UpdatePortPayload) -> Result<(), AppError> {
    run_blocking(move || {
        let target = resolve_config_path(payload.ssh.config_path.as_deref());
        let client = SshClient::connect(&payload.ssh)?;
        let raw = client.read_file(&target)?;

        let mut config: serde_json::Value = serde_json::from_str(&raw)
            .map_err(|e| AppError::new(AppErrorCode::JsonParseFailed, e.to_string()))?;

        // Update the FIRST inbound's port (main entry point)
        if let Some(inbounds) = config.get_mut("inbounds").and_then(serde_json::Value::as_array_mut) {
            if let Some(first_inbound) = inbounds.get_mut(0) {
                first_inbound["port"] = serde_json::json!(payload.new_port);
            } else {
                return Err(AppError::new(AppErrorCode::General, "Configuration has empty inbounds list"));
            }
        } else {
            return Err(AppError::new(AppErrorCode::General, "Configuration has no inbounds section"));
        }

        let updated_raw = serde_json::to_string_pretty(&config)
            .map_err(|e| AppError::new(AppErrorCode::JsonParseFailed, e.to_string()))?;

        client.write_file_atomic(&target, &updated_raw)?;
        let _ = client.exec("systemctl restart v2ray");

        Ok(())
    })
    .await
}
