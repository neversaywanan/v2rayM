import { useState, useCallback, useMemo } from "react";
import { api } from "../api/tauri";
import type { ConnectPayload, ParsedNode, ProxyMode, V2RayConfigLite } from "../types/api";
import { translations, Language } from "../locales/i18n";
import { sanitizeTag } from "../utils/tag";

// ─── Persistence keys ────────────────────────────────────────────────────────
const CONN_KEY = "v2ray_client_connection";
const SUB_URL_KEY = "v2ray_subscription_url";
const LANG_KEY = "v2ray_lang";
const TAG_MAP_PREFIX = "v2ray_tag_name_map";

interface SavedConnection {
  host: string;
  port: number;
  username: string;
  configPath?: string;
}

function loadJson<T>(key: string, fallback: T): T {
  try {
    const raw = localStorage.getItem(key);
    if (raw) return JSON.parse(raw) as T;
  } catch {}
  return fallback;
}

function saveJson(key: string, value: unknown) {
  try { localStorage.setItem(key, JSON.stringify(value)); } catch {}
}

const savedConn = loadJson<SavedConnection>(CONN_KEY, { host: "", port: 22, username: "", configPath: "" });
const savedSubUrl = localStorage.getItem(SUB_URL_KEY) ?? "";

const initialConnection: ConnectPayload = {
  host: savedConn.host,
  port: savedConn.port,
  username: savedConn.username,
  password: "",
  configPath: savedConn.configPath || "",
};

// ─── Loading task labels ─────────────────────────────────────────────────────
export type LoadingTask =
  | "connect"
  | "fetch"
  | "apply"
  | "switch"
  | "test"
  | null;

// ─── Store ───────────────────────────────────────────────────────────────────
export function useAppStore() {
  const [connected, setConnected] = useState(false);
  const [connection, setConnectionState] = useState<ConnectPayload>({ ...initialConnection });
  const [subscriptionUrl, setSubscriptionUrlState] = useState(savedSubUrl);
  const [mode, setMode] = useState<ProxyMode>("rule");
  const [parsedNodes, setParsedNodes] = useState<ParsedNode[]>([]);
  const [loadingTask, setLoadingTask] = useState<LoadingTask>(null);
  const [rawConfig, setRawConfig] = useState("");
  const [structuredConfig, setStructuredConfig] = useState<V2RayConfigLite | null>(null);
  const [usedPath, setUsedPath] = useState("");
  const [statusLog, setStatusLog] = useState("");
  const [errorText, setErrorText] = useState("");
  const [proxyLatencyMs, setProxyLatencyMs] = useState<number | null>(null);
  const [tagNameMap, setTagNameMap] = useState<Record<string, string>>(() => {
    const key = `${TAG_MAP_PREFIX}_${savedConn.host}_${savedConn.port}`;
    return loadJson<Record<string, string>>(key, {});
  });
  const [lang, setLangState] = useState<Language>((localStorage.getItem(LANG_KEY) as Language) || "en");
  const [toasts, setToasts] = useState<any[]>([]);

  const t = useMemo(() => translations[lang], [lang]);

  const setLang = useCallback((l: Language) => {
    setLangState(l);
    localStorage.setItem(LANG_KEY, l);
  }, []);

  const loading = loadingTask !== null;

  const persistTagMap = useCallback((next: Record<string, string>, conn?: ConnectPayload) => {
    const host = (conn?.host ?? connection.host ?? savedConn.host) || "";
    const port = (conn?.port ?? connection.port ?? savedConn.port) || 22;
    const key = `${TAG_MAP_PREFIX}_${host}_${port}`;
    setTagNameMap(next);
    saveJson(key, next);
  }, [connection.host, connection.port]);

  const rememberNodeNames = useCallback((nodes: ParsedNode[], conn?: ConnectPayload) => {
    if (!nodes || nodes.length === 0) return;
    const next = { ...tagNameMap };
    for (const n of nodes) {
      const tag = sanitizeTag(n.name);
      if (tag) next[tag] = n.name;
    }
    persistTagMap(next, conn);
  }, [tagNameMap, persistTagMap]);

  const displayNameForTag = useCallback((tag?: string | null) => {
    if (!tag) return undefined;
    return tagNameMap[tag] ?? undefined;
  }, [tagNameMap]);

  const displayNameForActiveTag = useCallback(() => {
    const tag = structuredConfig?.activeOutboundTag;
    if (!tag) return undefined;

    // Prefer server-provided display name (meta.displayName), fall back to local map.
    const serverName = structuredConfig?.outbounds?.find((o: any) => o.tag === tag)?.displayName;
    return serverName || tagNameMap[tag];
  }, [structuredConfig, tagNameMap]);

  // ── Toast Utility ────────────────────────────────────────────────────────
  const notify = useCallback((message: string, type: "success" | "error" | "info" | "warning" = "info") => {
    const id = Date.now() + Math.random();
    setToasts((prev) => [...prev, { id, message, type, exiting: false }]);

    // Start exit animation after 3.5s
    setTimeout(() => {
      setToasts((prev) => prev.map(t => t.id === id ? { ...t, exiting: true } : t));
    }, 3500);

    // Remove after 4s
    setTimeout(() => {
      setToasts((prev) => prev.filter((t) => t.id !== id));
    }, 4000);
  }, []);

  // Persist subscription URL change
  const setSubscriptionUrl = useCallback((url: string) => {
    setSubscriptionUrlState(url);
    try { localStorage.setItem(SUB_URL_KEY, url); } catch {}
  }, []);

  // ── Connect ──────────────────────────────────────────────────────────────
  const connect = useCallback(async (conn: ConnectPayload) => {
    setLoadingTask("connect");
    setErrorText("");
    try {
      const result = await api.connectAndFetchConfig(conn);
      setRawConfig(result.rawText);
      setStructuredConfig(result.parsed ?? null);
      setUsedPath(result.usedPath);
      setConnectionState(conn);
      setConnected(true);
      saveJson(CONN_KEY, { host: conn.host, port: conn.port, username: conn.username, configPath: conn.configPath });

       // Load persisted tag->name map for this server connection
      const key = `${TAG_MAP_PREFIX}_${conn.host}_${conn.port}`;
      setTagNameMap(loadJson<Record<string, string>>(key, {}));
      return { success: true };
    } catch (e) {
      const error = formatError(e);
      setErrorText(error);
      notify(`${t.connFailed}: ${error}`, "error");
      return { success: false, error };
    } finally {
      setLoadingTask(null);
    }
  }, [notify]);

  // ── Refresh config from server ───────────────────────────────────────────
  const refreshConfig = useCallback(async (conn: ConnectPayload) => {
    try {
      const updated = await api.connectAndFetchConfig(conn);
      setRawConfig(updated.rawText);
      setStructuredConfig(updated.parsed ?? null);
    } catch {}
  }, []);

  // ── Fetch subscription → auto apply all nodes ────────────────────────────
  const fetchSubscriptionNodes = useCallback(async (urlOverride?: string) => {
    const targetUrl = (urlOverride ?? subscriptionUrl).trim();
    if (!targetUrl) return { success: false, error: t.subUrlError };

    setLoadingTask("fetch");
    setSubscriptionUrl(targetUrl);
    setErrorText("");
    try {
      // 1. Fetch and parse subscription
      const result = await api.fetchSubscription(targetUrl);
      setParsedNodes(result.nodes);
      rememberNodeNames(result.nodes);
      setStatusLog(`${t.subParseComplete}: ${result.nodes.length} ${t.nodes}, ${t.invalid} ${result.invalidCount}`);

      if (result.nodes.length === 0) {
        return { success: true, count: 0 };
      }

      // 2. Auto-apply ALL nodes to server config (keeps existing base config)
      setLoadingTask("apply");
      await api.applyRemoteConfig({
        ssh: connection,
        targetPath: connection.configPath ?? "",
        selectedNodes: result.nodes,
        mode,
        baseConfigRaw: rawConfig,
      });

      // Ensure mapping stays up-to-date after syncing
      rememberNodeNames(result.nodes);
      setStatusLog(`${t.subSuccess}: ${result.nodes.length} ${t.nodes}`);
      notify(`${t.subSuccess}`, "success");

      // 3. Refresh the displayed config
      await refreshConfig(connection);
      return { success: true, count: result.nodes.length };
    } catch (e) {
      const error = formatError(e);
      setErrorText(error);
      notify(`${t.subFailed}: ${error}`, "error");
      return { success: false, error };
    } finally {
      setLoadingTask(null);
    }
  }, [subscriptionUrl, connection, mode, rawConfig, refreshConfig, setSubscriptionUrl, notify]);

  // ── Switch active outbound (update routing rules only) ───────────────────
  const switchOutbound = useCallback(async (tag: string) => {
    setLoadingTask("switch");
    setErrorText("");
    try {
      await api.switchActiveOutbound({ ssh: connection, tag });
      // Update local state immediately for snappy UI, then confirm from server
      setStructuredConfig((prev) =>
        prev ? { ...prev, activeOutboundTag: tag } : prev
      );
      setStatusLog(`${t.switchedTo}: ${tag}`);
      notify(`${t.switchedTo}: ${tag}`, "success");
      // Refresh to confirm server state
      await refreshConfig(connection);
      return { success: true };
    } catch (e) {
      const error = formatError(e);
      setErrorText(error);
      notify(`${t.switchFailed}: ${error}`, "error");
      return { success: false, error };
    } finally {
      setLoadingTask(null);
    }
  }, [connection, refreshConfig, notify]);

  // ── Apply a single selected node ─────────────────────────────────────────
  const applyConfig = useCallback(async (node: ParsedNode) => {
    setLoadingTask("apply");
    setErrorText("");
    try {
      const result = await api.applyRemoteConfig({
        ssh: connection,
        targetPath: connection.configPath ?? "",
        selectedNodes: [node],
        mode,
        baseConfigRaw: rawConfig,
      });
      setStatusLog(`${t.applySuccess}\n${t.backup}: ${result.backupPath}\n${result.statusSummary}`);
      await refreshConfig(connection);
      return { success: true };
    } catch (e) {
      const error = formatError(e);
      setErrorText(error);
      return { success: false, error };
    } finally {
      setLoadingTask(null);
    }
  }, [connection, mode, rawConfig, refreshConfig]);

  // ── Test proxy ───────────────────────────────────────────────────────────
  const testProxy = useCallback(async (opts?: { silent?: boolean }) => {
    const silent = !!opts?.silent;
    if (!silent) {
      setLoadingTask("test");
    }
    setErrorText("");
    try {
      const result = await api.testProxyConnection(connection);
      setProxyLatencyMs(parseProxyLatencyMs(result));
      setStatusLog(`${t.testResult}:\n${result}`);
      return { success: true, result };
    } catch (e) {
      const error = formatError(e);
      setErrorText(error);
      setProxyLatencyMs(null);
      return { success: false, error };
    } finally {
      if (!silent) {
        setLoadingTask(null);
      }
    }
  }, [connection]);

  // ── Update Inbound Port ──────────────────────────────────────────────────
  const updatePort = useCallback(async (newPort: number) => {
    setLoadingTask("apply");
    setErrorText("");
    try {
      await api.updateInboundPort({ ssh: connection, newPort });
      setStatusLog(`${t.portSaved}: ${newPort}`);
      notify(`${t.portSaved}: ${newPort}`, "success");
      await refreshConfig(connection);
      return { success: true };
    } catch (e) {
      const error = formatError(e);
      setErrorText(error);
      notify(`${t.portFailed}: ${error}`, "error");
      return { success: false, error };
    } finally {
      setLoadingTask(null);
    }
  }, [connection, refreshConfig, notify]);

  // ── Disconnect ───────────────────────────────────────────────────────────
  const disconnect = useCallback(() => {
    setConnected(false);
    setRawConfig("");
    setStructuredConfig(null);
    setParsedNodes([]);
    setStatusLog("");
    setErrorText("");
    setProxyLatencyMs(null);
    setLoadingTask(null);
  }, []);

  return {
    connected,
    connection,
    subscriptionUrl,
    setSubscriptionUrl,
    mode,
    setMode,
    parsedNodes,
    loading,
    loadingTask,
    rawConfig,
    structuredConfig,
    usedPath,
    statusLog,
    errorText,
    proxyLatencyMs,
    tagNameMap,
    displayNameForTag,
    displayNameForActiveTag,
    toasts,
    notify,
    lang,
    setLang,
    t,
    connect,
    fetchSubscriptionNodes,
    switchOutbound,
    applyConfig,
    testProxy,
    updatePort,
    disconnect,
  };
}

// ─── Helpers ──────────────────────────────────────────────────────────────────
function formatError(e: unknown): string {
  if (typeof e === "string") return e;
  if (e && typeof e === "object") {
    const obj = e as Record<string, unknown>;
    if ("message" in obj && typeof obj.message === "string") return obj.message;
    if ("error" in obj && typeof obj.error === "string") return obj.error;
    if ("code" in obj && "message" in obj) return `[${obj.code}] ${obj.message}`;
    try { return JSON.stringify(obj); } catch { return String(obj); }
  }
  return String(e);
}

function parseProxyLatencyMs(resultText: string): number | null {
  const statusMatch = resultText.match(/SOCKS5 proxy HTTP Status:\s*(\d+)/i);
  const statusCode = statusMatch ? Number(statusMatch[1]) : 0;
  if (statusCode !== 200 && statusCode !== 204) {
    return null;
  }

  const proxyBlockMatch = resultText.match(/SOCKS5 proxy HTTP Status:[^\n]*\nTime Total:\s*([0-9.]+)s/i);
  if (!proxyBlockMatch) {
    return null;
  }

  const seconds = Number(proxyBlockMatch[1]);
  if (!Number.isFinite(seconds) || seconds <= 0) {
    return null;
  }

  return Math.max(1, Math.round(seconds * 1000));
}
