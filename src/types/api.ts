export type ProxyMode = "direct" | "global" | "rule";
export type NodeProtocol = "vmess" | "vless" | "trojan" | "ss" | "ssr";

export interface ConnectPayload {
  host: string;
  port: number;
  username: string;
  password: string;
  configPath?: string;
}

export interface ParsedNode {
  id: string;
  name: string;
  protocol: NodeProtocol;
  server: string;
  port: number;
  uuid: string;
  security?: string;
  network?: string;
  tls?: boolean;
  sni?: string;
  alpn?: string[];
}

export interface InboundLite {
  port?: number;
  protocol?: string;
  listen?: string;
}

export interface OutboundLite {
  protocol?: string;
  tag?: string;
  displayName?: string;
  server?: string;
  port?: number;
}

export interface V2RayConfigLite {
  inbounds: InboundLite[];
  outbounds: OutboundLite[];
  routingRulesCount: number;
  activeOutboundTag?: string;
}

export interface FetchResult {
  rawText: string;
  parsed?: V2RayConfigLite;
  usedPath: string;
  fetchedAt: string;
}

export interface SubscriptionParseResult {
  nodes: ParsedNode[];
  invalidCount: number;
  invalidSamples: string[];
}

export interface ApplyPayload {
  ssh: ConnectPayload;
  targetPath: string;
  selectedNodes: ParsedNode[];
  mode: ProxyMode;
  baseConfigRaw?: string;
}

export interface ApplyResult {
  backupPath: string;
  usedPath: string;
  restartOk: boolean;
  statusSummary: string;
  appliedAt: string;
}

export interface SwitchOutboundPayload {
  ssh: ConnectPayload;
  tag: string;
}

export interface UpdatePortPayload {
  ssh: ConnectPayload;
  newPort: number;
}

export interface AppError {
  code: string;
  message: string;
  suggestion?: string;
}
