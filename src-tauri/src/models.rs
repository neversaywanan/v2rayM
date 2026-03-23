use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProxyMode {
    Direct,
    Global,
    Rule,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NodeProtocol {
    Vmess,
    Vless,
    Trojan,
    Ss,
    Ssr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectPayload {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub config_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedNode {
    pub id: String,
    pub name: String,
    pub protocol: NodeProtocol,
    pub server: String,
    pub port: u16,
    pub uuid: String,
    pub security: Option<String>,
    pub network: Option<String>,
    pub tls: Option<bool>,
    pub sni: Option<String>,
    pub alpn: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionParseResult {
    pub nodes: Vec<ParsedNode>,
    pub invalid_count: usize,
    pub invalid_samples: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V2RayConfigLite {
    pub inbounds: Vec<InboundLite>,
    pub outbounds: Vec<OutboundLite>,
    pub routing_rules_count: usize,
    pub active_outbound_tag: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboundLite {
    pub port: Option<u16>,
    pub protocol: Option<String>,
    pub listen: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutboundLite {
    pub protocol: Option<String>,
    pub tag: Option<String>,
    pub display_name: Option<String>,
    pub server: Option<String>,
    pub port: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchResult {
    pub raw_text: String,
    pub parsed: Option<V2RayConfigLite>,
    pub used_path: String,
    pub fetched_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplyPayload {
    pub ssh: ConnectPayload,
    pub target_path: String,
    pub selected_nodes: Vec<ParsedNode>,
    pub mode: ProxyMode,
    pub base_config_raw: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplyResult {
    pub backup_path: String,
    pub used_path: String,
    pub restart_ok: bool,
    pub status_summary: String,
    pub applied_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwitchOutboundPayload {
    pub ssh: ConnectPayload,
    pub tag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePortPayload {
    pub ssh: ConnectPayload,
    pub new_port: u16,
}
