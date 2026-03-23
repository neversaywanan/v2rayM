import { useState, useEffect } from "react";
import { useAppStore } from "./stores/appStore";
import type { ProxyMode } from "./types/api";
import { sanitizeTag } from "./utils/tag";

type ViewType = "general" | "proxies" | "logs";

// ─── Task labels shown in the global loading overlay ───────────────────────
function getTaskLabel(task: string, t: any) {
  const labels: Record<string, string> = {
    connect: t.connecting,
    fetch: t.syncing,
    apply: t.saving,
    test: t.testing,
  };
  return labels[task] || t.unknown;
}

// ─── Global Loading Overlay ─────────────────────────────────────────────────
function GlobalLoadingOverlay({ task, t }: { task: string | null; t: any }) {
  if (!task) return null;
  return (
    <div style={{
      position: "fixed",
      inset: 0,
      backgroundColor: "rgba(15, 23, 42, 0.6)",
      backdropFilter: "blur(4px)",
      display: "flex",
      flexDirection: "column",
      alignItems: "center",
      justifyContent: "center",
      zIndex: 9999,
      animation: "fadeIn 0.15s ease-out",
    }}>
      <div style={{
        backgroundColor: "#ffffff",
        borderRadius: 16,
        padding: "32px 48px",
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        gap: 20,
        boxShadow: "0 25px 50px -12px rgba(0,0,0,0.4)",
      }}>
        <Spinner size={40} />
        <div style={{ fontWeight: 600, fontSize: 15, color: "#1e293b" }}>
          {getTaskLabel(task, t)}
        </div>
      </div>
    </div>
  );
}

function Spinner({ size = 24, color = "var(--primary)" }: { size?: number; color?: string }) {
  return (
    <svg
      width={size} height={size}
      viewBox="0 0 24 24" fill="none"
      stroke={color} strokeWidth="2.5"
      strokeLinecap="round"
      style={{ animation: "spin 0.9s linear infinite", transformOrigin: "center" }}
    >
      <circle cx="12" cy="12" r="9" opacity="0.25" />
      <path d="M12 3a9 9 0 0 1 9 9" />
    </svg>
  );
}

// ─── Toast System ───────────────────────────────────────────────────────────
function ToastContainer({ toasts }: { toasts: any[] }) {
  if (toasts.length === 0) return null;
  return (
    <div className="toast-container">
      {toasts.map((toast) => (
        <div key={toast.id} className={`toast toast-${toast.type} ${toast.exiting ? "exit" : ""}`}>
          <div style={{ flexShrink: 0 }}>
            {toast.type === "success" && <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round"><polyline points="20 6 9 17 4 12"/></svg>}
            {toast.type === "error" && <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round"><circle cx="12" cy="12" r="10"/><line x1="15" y1="9" x2="9" y2="15"/><line x1="9" y1="9" x2="15" y2="15"/></svg>}
            {toast.type === "warning" && <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>}
            {toast.type === "info" && <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round"><circle cx="12" cy="12" r="10"/><line x1="12" y1="16" x2="12" y2="12"/><line x1="12" y1="8" x2="12.01" y2="8"/></svg>}
          </div>
          <div style={{ flex: 1 }}>{toast.message}</div>
        </div>
      ))}
    </div>
  );
}

// ─── App ────────────────────────────────────────────────────────────────────
export default function App() {
  const store = useAppStore();
  const [activeView, setActiveView] = useState<ViewType>("general");
  const [autoTested, setAutoTested] = useState(false);

  useEffect(() => {
    if (!store.connected) {
      setAutoTested(false);
      return;
    }
    if (autoTested) {
      return;
    }

    setAutoTested(true);
    void store.testProxy({ silent: true });
  }, [store.connected, store.testProxy, autoTested]);

  if (!store.connected) {
    return (
      <>
        <GlobalLoadingOverlay task={store.loadingTask} t={store.t} />
        <ToastContainer toasts={store.toasts} />
        <ConnectPage store={store} />
      </>
    );
  }

  return (
    <>
      <GlobalLoadingOverlay task={store.loadingTask} t={store.t} />
      <ToastContainer toasts={store.toasts} />
      <div style={{ display: "flex", height: "100vh", width: "100%" }}>
        <Sidebar activeView={activeView} setActiveView={setActiveView} store={store} />
        <main className="main-content">
          <div className="content-header">
            <h1 className="page-title">{getViewTitle(activeView, store.t)}</h1>
            <div style={{ display: "flex", alignItems: "center", gap: 8, fontSize: 14, color: "var(--text-muted)" }}>
              <span style={{ width: 8, height: 8, borderRadius: "50%", backgroundColor: "#22c55e" }}></span>
              {store.connection.host}:{store.connection.port}
            </div>
          </div>

          <div className="view-container">
            {activeView === "general" && <GeneralView store={store} />}
            {activeView === "proxies" && <ProxiesView store={store} />}
            {activeView === "logs" && <LogsView store={store} />}
          </div>
        </main>
      </div>
    </>
  );
}

function getViewTitle(view: ViewType, t: any) {
  switch (view) {
    case "general":  return t.navGeneral;
    case "proxies":  return t.navProxies;
    case "logs":     return t.navLogs;
  }
}

// ─── Sidebar ────────────────────────────────────────────────────────────────
function Sidebar({ activeView, setActiveView, store }: {
  activeView: ViewType;
  setActiveView: (v: ViewType) => void;
  store: any;
}) {
  const { t } = store;
  return (
    <aside className="sidebar">
      <div className="sidebar-header">
        <div className="sidebar-title">
          <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
            <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5" />
          </svg>
          {t.appName}
        </div>
      </div>

      <nav className="nav-links">
        <NavItem active={activeView === "general"} onClick={() => setActiveView("general")} label={t.navGeneral}
          icon={<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><rect x="3" y="3" width="7" height="7"/><rect x="14" y="3" width="7" height="7"/><rect x="14" y="14" width="7" height="7"/><rect x="3" y="14" width="7" height="7"/></svg>}
        />
        <NavItem active={activeView === "proxies"} onClick={() => setActiveView("proxies")} label={t.navProxies}
          icon={<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><polyline points="16 18 22 12 16 6"/><polyline points="8 6 2 12 8 18"/></svg>}
        />
        <NavItem active={activeView === "logs"} onClick={() => setActiveView("logs")} label={t.navLogs}
          icon={<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z"/><polyline points="13 2 13 9 20 9"/></svg>}
        />
      </nav>

      <div className="sidebar-footer">
        <button
          className="nav-item"
          style={{ width: "100%", border: "none", background: "none", cursor: "pointer" }}
          onClick={store.disconnect}
          disabled={store.loading}
        >
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
            <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/>
            <polyline points="16 17 21 12 16 7"/>
            <line x1="21" y1="12" x2="9" y2="12"/>
          </svg>
          {t.logout}
        </button>
      </div>
    </aside>
  );
}

function NavItem({ active, onClick, label, icon }: { active: boolean; onClick: () => void; label: string; icon: React.ReactNode }) {
  return (
    <div className={`nav-item ${active ? "active" : ""}`} onClick={onClick}>
      {icon}
      {label}
    </div>
  );
}

// ─── General View ────────────────────────────────────────────────────────────
function GeneralView({ store }: { store: any }) {
  const currentPort = store.structuredConfig?.inbounds?.[0]?.port ?? 0;
  const activeNode = store.structuredConfig?.activeOutboundTag ?? "未知";
  const activeNodeName = store.displayNameForActiveTag?.() || store.displayNameForTag?.(store.structuredConfig?.activeOutboundTag);
  const [editingPort, setEditingPort] = useState<string>("");
  const isUpdating = store.loadingTask === "apply";
  const isTestingLatency = store.loadingTask === "test";

  // Initialize editingPort when currentPort changes
  useEffect(() => {
    if (currentPort) {
      setEditingPort(currentPort.toString());
    }
  }, [currentPort]);

  const handleUpdatePort = async () => {
    const portNum = parseInt(editingPort);
    if (isNaN(portNum) || portNum < 1 || portNum > 65535) {
      store.notify(store.t.invalidPort, "warning");
      return;
    }
    await store.updatePort(portNum);
  };

  return (
    <div className="view-content">
      {/* Configuration Details */}
      <div className="card">
        <h3 style={{ marginTop: 0 }}>{store.t.coreSettings}</h3>
        <div style={{ display: "flex", flexDirection: "column", gap: 20 }}>
          {/* Active Node Display */}
          <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
            <div>
              <div style={{ color: "var(--text-muted)", fontSize: 13, marginBottom: 4 }}>{store.t.activeNode}</div>
              <div style={{ fontWeight: 600, fontSize: 16, color: "var(--primary)" }}>
                {activeNodeName || (activeNode === "未知" ? store.t.unknown : activeNode)}
              </div>
              <div style={{ marginTop: 6, fontSize: 12, color: "var(--text-muted)" }}>
                {store.t.activeLatency}: {isTestingLatency
                  ? store.t.testing
                  : typeof store.proxyLatencyMs === "number"
                  ? `${store.proxyLatencyMs}ms`
                  : "--"}
              </div>
            </div>
            <div style={{
              width: 10, height: 10, borderRadius: "50%",
              backgroundColor: activeNode !== "未知" ? "#22c55e" : "#94a3b8",
              boxShadow: activeNode !== "未知" ? "0 0 0 4px rgba(34,197,94,0.15)" : "none"
            }} />
          </div>

          {/* Port Edit Section */}
          <div>
            <div style={{ color: "var(--text-muted)", fontSize: 13, marginBottom: 8 }}>{store.t.inboundPort}</div>
            <div style={{ display: "flex", gap: 12 }}>
              <input
                className="input"
                type="number"
                value={editingPort}
                onChange={(e) => setEditingPort(e.target.value)}
                placeholder="Port..."
                disabled={store.loading}
                style={{ maxWidth: 120 }}
              />
              <button
                className="btn btn-primary"
                onClick={handleUpdatePort}
                disabled={store.loading || editingPort === currentPort?.toString()}
                style={{ gap: 6 }}
              >
                {isUpdating ? <><Spinner size={14} color="#fff" /> {store.t.saving}</> : store.t.savePort}
              </button>
            </div>
          </div>
        </div>
      </div>

      {/* Status cards */}
      <div className="card">
        <h3 style={{ marginTop: 0 }}>{store.t.summary}</h3>
        <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr 1fr", gap: 24 }}>
          <StatCard label={store.t.inboundCount} value={store.structuredConfig?.inbounds.length ?? 0} />
          <StatCard label={store.t.outboundCount} value={store.structuredConfig?.outbounds.length ?? 0} />
          <StatCard label={store.t.routingRules} value={store.structuredConfig?.routingRulesCount ?? 0} />
        </div>
        <div style={{ marginTop: 16, fontSize: 12, color: "var(--text-muted)" }}>
          {store.t.remotePath}: {store.usedPath || "—"}
        </div>
      </div>

      {/* Proxy mode */}
      <div className="card">
        <h3 style={{ marginTop: 0 }}>{store.t.proxyMode}</h3>
        <div style={{ display: "flex", gap: 12 }}>
          {(["direct", "global", "rule"] as ProxyMode[]).map((m) => (
            <button
              key={m}
              className={`btn ${store.mode === m ? "btn-primary" : "btn-outline"}`}
              style={{ flex: 1 }}
              onClick={() => store.setMode(m)}
              disabled={store.loading}
            >
              {m === "direct" ? store.t.modeDirect : m === "global" ? store.t.modeGlobal : store.t.modeRule}
            </button>
          ))}
        </div>
      </div>
    </div>
  );
}

// ─── Connectivity Card (shared) ───────────────────────────────────────────────
function ConnectivityCard({ store }: { store: any }) {
  const [testResult, setTestResult] = useState<{ success: boolean; message: string; details?: string; ip?: string; loc?: string } | null>(null);
  const isTesting = store.loadingTask === "test";

  const handleTestProxy = async () => {
    setTestResult(null);
    const result = await store.testProxy();
    if (result.success) {
      const text = result.result as string;
      const isProxyOk = text.includes("SOCKS5 proxy HTTP Status: 200");
      const isServiceRunning = text.includes("Service Status: active");
      
      // Parse IP and Location from cloudflare trace (matching Python multiline logic)
      let foundIp = "";
      let foundLoc = "";
      
      const ipMatch = text.match(/^ip=(.+)$/m);
      if (ipMatch) foundIp = ipMatch[1];
      
      const locMatch = text.match(/^loc=(.+)$/m);
      if (locMatch) foundLoc = locMatch[1];

      const message = isProxyOk
        ? store.t.testSuccess
        : isServiceRunning
        ? store.t.testPartial
        : store.t.testFailed;
      setTestResult({ success: isProxyOk, message, details: text, ip: foundIp, loc: foundLoc });
    } else {
      setTestResult({ success: false, message: `${store.t.testGeneralError}: ${result.error}` });
    }
  };

  const getCountryName = (code: string) => {
    const map: Record<string, string> = {
      'HK': store.lang === 'zh' ? '香港' : 'Hong Kong',
      'TW': store.lang === 'zh' ? '台湾' : 'Taiwan',
      'JP': store.lang === 'zh' ? '日本' : 'Japan',
      'KR': store.lang === 'zh' ? '韩国' : 'South Korea',
      'US': store.lang === 'zh' ? '美国' : 'USA',
      'SG': store.lang === 'zh' ? '新加坡' : 'Singapore',
      'GB': store.lang === 'zh' ? '英国' : 'UK',
      'DE': store.lang === 'zh' ? '德国' : 'Germany',
      'FR': store.lang === 'zh' ? '法国' : 'France',
      'CA': store.lang === 'zh' ? '加拿大' : 'Canada',
      'AU': store.lang === 'zh' ? '澳大利亚' : 'Australia',
      'RU': store.lang === 'zh' ? '俄罗斯' : 'Russia',
      'CN': store.lang === 'zh' ? '中国' : 'China'
    };
    return map[code] || code;
  };

  return (
    <div className="card">
      <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between", marginBottom: testResult ? 16 : 0 }}>
        <div>
          <h3 style={{ margin: 0, marginBottom: 4 }}>{store.t.latencyTest}</h3>
          <p style={{ color: "var(--text-muted)", fontSize: 13, margin: 0 }}>{store.t.latencyDesc}</p>
        </div>
        <button
          className="btn btn-outline"
          onClick={handleTestProxy}
          disabled={store.loading}
          style={{ gap: 8, flexShrink: 0 }}
        >
          {isTesting ? <><Spinner size={13} /> {store.t.testing}</> : store.t.startTest}
        </button>
      </div>
      {testResult && (
        <div style={{
          padding: 12, borderRadius: 8,
          backgroundColor: testResult.success ? "#f0fdf4" : "#fef2f2",
          border: `1px solid ${testResult.success ? "#bbf7d0" : "#fecaca"}`,
          color: testResult.success ? "#166534" : "#991b1b",
        }}>
          {testResult.success ? (
            <div style={{ display: "flex", flexDirection: "column", gap: 8 }}>
              <div style={{ display: "flex", alignItems: "center", gap: 8 }}>
                <span style={{ fontSize: 13, background: "var(--primary)", color: "white", padding: "2px 8px", borderRadius: 4, fontWeight: 700 }}>
                  {testResult.loc ? `[${testResult.loc}] ${getCountryName(testResult.loc)}` : store.t.unknown}
                </span>
                <span style={{ fontSize: 11, opacity: 0.6 }}>{store.t.outboundIp}</span>
              </div>
              <div style={{ 
                fontSize: 14, 
                background: "rgba(0,0,0,0.06)", 
                padding: "8px 12px", 
                borderRadius: 6, 
                fontFamily: "monospace", 
                fontWeight: 700,
                wordBreak: "break-all",
                color: "var(--text-main)"
              }}>
                {testResult.ip || "N/A"}
              </div>
            </div>
          ) : (
            <div style={{ fontWeight: 600, display: "flex", alignItems: "center", gap: 6 }}>
              <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3"><circle cx="12" cy="12" r="10"/><line x1="15" y1="9" x2="9" y2="15"/><line x1="9" y1="9" x2="15" y2="15"/></svg>
              {testResult.message}
            </div>
          )}
        </div>
      )}
    </div>
  );
}

function StatCard({ label, value }: { label: string; value: number }) {
  return (
    <div>
      <div style={{ color: "var(--text-muted)", fontSize: 13, marginBottom: 4 }}>{label}</div>
      <div style={{ fontSize: 28, fontWeight: 700 }}>{value}</div>
    </div>
  );
}

// ─── Proxies View ────────────────────────────────────────────────────────────
function ProxiesView({ store }: { store: any }) {
  const [url, setUrl] = useState(store.subscriptionUrl);
  const [fetchMsg, setFetchMsg] = useState<{ text: string; ok: boolean } | null>(null);
  const isFetching = store.loadingTask === "fetch" || store.loadingTask === "apply";
  const isSwitching = store.loadingTask === "switch";

  const handleFetch = async () => {
    if (!url.trim()) { setFetchMsg({ text: store.t.subUrlError, ok: false }); return; }
    setFetchMsg(null);
    const res = await store.fetchSubscriptionNodes(url);
    if (res.success) {
      setFetchMsg({ text: `✓ ${res.count} ${store.t.nodes}`, ok: true });
    } else {
      setFetchMsg({ text: `${store.t.subFailed}: ${res.error}`, ok: false });
    }
  };

  const handleNodeClick = async (tag: string) => {
    if (store.loading) return;
    await store.switchOutbound(tag);
  };

  // Active tag from server routing rules
  const activeTag = store.structuredConfig?.activeOutboundTag;

  // Source of truth for display:
  // - If subscription nodes have been fetched, show them (as they were synced to server)
  // - Otherwise fall back to showing the server's current outbounds from config
  const hasSubscription = store.parsedNodes.length > 0;

  const subscriptionMode = hasSubscription;

  return (
    <div className="view-content">
      {/* Connectivity test — top of proxies page */}
      <ConnectivityCard store={store} />

      {/* Subscription bar */}
      <div className="card">
        <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between", marginBottom: 12 }}>
          <h3 style={{ margin: 0 }}>{store.t.updateSub}</h3>
          {hasSubscription && (
            <span style={{ fontSize: 12, color: "#10b981", fontWeight: 500, display: "flex", alignItems: "center", gap: 4 }}>
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3"><polyline points="20 6 9 17 4 12"/></svg>
              {store.parsedNodes.length} {store.t.subNodes}
            </span>
          )}
        </div>
        <div style={{ display: "flex", gap: 12 }}>
          <input
            className="input"
            type="text"
            value={url}
            onChange={(e) => { setUrl(e.target.value); store.setSubscriptionUrl(e.target.value); }}
            placeholder={store.t.subUrlPlaceholder}
            disabled={store.loading}
          />
          <button
            className="btn btn-primary"
            onClick={handleFetch}
            disabled={store.loading}
            style={{ minWidth: 120, gap: 6, flexShrink: 0 }}
          >
            {isFetching
              ? <><Spinner size={14} color="#fff" /> {store.t.syncing}</>
              : store.t.updateSub
            }
          </button>
        </div>
        {fetchMsg && (
          <div style={{ marginTop: 10, fontSize: 13, color: fetchMsg.ok ? "#059669" : "#dc2626" }}>
            {fetchMsg.text}
          </div>
        )}
      </div>

      {/* Unified node list */}
      <section>
        <div style={{ display: "flex", alignItems: "center", gap: 8, marginBottom: 16 }}>
          <SectionTitle
            color={subscriptionMode ? "#10b981" : "var(--primary)"}
            title={subscriptionMode
              ? `${store.t.subNodes} (${store.parsedNodes.length}) — ${store.t.clickToSwitch}`
              : `${store.t.serverNodes} — ${store.t.clickToSwitch}`
            }
          />
          {isSwitching && <Spinner size={14} />}
        </div>

        <div className="node-grid">
          {subscriptionMode
            ? store.parsedNodes.map((node: any) => {
                // The tag that will be used on server (sanitized name)
                const nodeTag = sanitizeTag(node.name);
                const isActive = activeTag === nodeTag;

                const baseTags = [
                  node.tls ? "TLS" : null,
                  node.network ?? null,
                ].filter(Boolean) as string[];

                const protoLower = String(node.protocol ?? "").toLowerCase();
                let protocolBadge: string | undefined = node.protocol;
                let tags = baseTags;

                // For vmess nodes, show "vmess" as a chip next to "ws" (same style),
                // and hide the top-right protocol badge.
                if (protoLower === "vmess") {
                  const next = [...baseTags];
                  const wsIndex = next.findIndex((t) => String(t).toLowerCase() === "ws");
                  if (wsIndex >= 0) {
                    next.splice(wsIndex + 1, 0, "vmess");
                  } else {
                    next.push("vmess");
                  }
                  tags = next;
                  protocolBadge = undefined;
                }

                return (
                  <NodeCard
                    key={node.id}
                    title={node.name}
                    subtitle={`${node.server}:${node.port}`}
                    protocol={protocolBadge}
                    isActive={isActive}
                    disabled={store.loading}
                    tags={tags}
                    onClick={() => handleNodeClick(nodeTag)}
                  />
                );
              })
            : (store.structuredConfig?.outbounds ?? [])
                .filter((o: any) => !["direct", "block"].includes(o.tag))
                .map((out: any, i: number) => {
                  const isActive = activeTag === out.tag;
                  const protoLower = String(out.protocol ?? "").toLowerCase();
                  const tags = protoLower === "vmess" ? ["vmess"] : [];
                  const protocolBadge = protoLower === "vmess" ? undefined : out.protocol;
                  return (
                    <NodeCard
                      key={i}
                      title={out.displayName || out.tag || store.t.unnamed}
                      subtitle={out.server ? `${out.server}:${out.port}` : store.t.localDirect}
                      protocol={protocolBadge}
                      isActive={isActive}
                      disabled={store.loading}
                      tags={tags}
                      onClick={() => out.tag && handleNodeClick(out.tag)}
                    />
                  );
                })
          }

          {!subscriptionMode && (!store.structuredConfig?.outbounds || store.structuredConfig.outbounds.filter((o: any) => !["direct","block"].includes(o.tag)).length === 0) && (
            <div style={{ color: "var(--text-muted)", fontSize: 14, gridColumn: "1/-1" }}>
              {store.t.noNodes}
            </div>
          )}
        </div>
      </section>
    </div>
  );
}

/** Mirror of Rust sanitize_tag logic for matching active tag */
// NOTE: tag sanitization lives in `src/utils/tag.ts` to match Rust behavior.

function NodeCard({ title, subtitle, protocol, isActive, disabled, tags, onClick }: {
  title: string;
  subtitle: string;
  protocol?: string;
  isActive: boolean;
  disabled: boolean;
  tags: string[];
  onClick: () => void;
}) {
  return (
    <div
      className={`node-card ${isActive ? "active" : ""}`}
      onClick={!disabled ? onClick : undefined}
      style={{ cursor: disabled ? "default" : "pointer", position: "relative" }}
    >
      {isActive && (
        <div style={{
          position: "absolute", top: 10, right: 10,
          width: 8, height: 8, borderRadius: "50%",
          backgroundColor: "#22c55e",
          boxShadow: "0 0 0 3px rgba(34,197,94,0.2)",
        }} />
      )}
      <div style={{ display: "flex", justifyContent: "space-between", alignItems: "flex-start", marginBottom: 10, paddingRight: isActive ? 16 : 0, gap: 8 }}>
        <div style={{ flex: 1, minWidth: 0, fontWeight: 600, fontSize: 14, overflow: "hidden", textOverflow: "ellipsis", whiteSpace: "nowrap" }}>
          {title}
        </div>
        {protocol && (
          <span className="protocol-badge" style={{ backgroundColor: getProtocolColor(protocol), flexShrink: 0 }}>
            {protocol}
          </span>
        )}
      </div>
      <div style={{ fontSize: 12, color: "var(--text-muted)", marginBottom: tags.length ? 8 : 0 }}>
        {subtitle}
      </div>
      {tags.length > 0 && (
        <div style={{ display: "flex", gap: 4 }}>
          {tags.map((t) => (
            <span key={t} style={{ fontSize: 10, padding: "2px 5px", border: "1px solid var(--border-color)", borderRadius: 4 }}>{t}</span>
          ))}
        </div>
      )}
    </div>
  );
}

function SectionTitle({ title, color }: { title: string; color: string }) {
  return (
    <h3 style={{ display: "flex", alignItems: "center", gap: 8, margin: 0 }}>
      <span style={{ width: 4, height: 16, backgroundColor: color, borderRadius: 2, flexShrink: 0 }} />
      {title}
    </h3>
  );
}


// ─── Logs View ───────────────────────────────────────────────────────────────
function LogsView({ store }: { store: any }) {
  return (
    <div className="view-content">
      <div className="card" style={{ padding: 0, overflow: "hidden", backgroundColor: "#1e293b", border: "none" }}>
        <div style={{ padding: "12px 16px", backgroundColor: "#0f172a", borderBottom: "1px solid #334155", display: "flex", justifyContent: "space-between", alignItems: "center" }}>
          <span style={{ color: "#94a3b8", fontSize: 12, fontFamily: "monospace" }}>app.log</span>
          <div style={{ display: "flex", gap: 6 }}>
            <span style={{ width: 8, height: 8, borderRadius: "50%", backgroundColor: "#ef4444" }} />
            <span style={{ width: 8, height: 8, borderRadius: "50%", backgroundColor: "#f59e0b" }} />
            <span style={{ width: 8, height: 8, borderRadius: "50%", backgroundColor: "#10b981" }} />
          </div>
        </div>
        <div style={{ padding: 20, height: "calc(100vh - 250px)", overflowY: "auto" }}>
          <pre style={{ color: "#e2e8f0", fontSize: 13, margin: 0, whiteSpace: "pre-wrap", lineHeight: 1.7 }}>
            {store.statusLog || store.t.waitingLogs}
            {store.errorText ? `\n\n[ERROR] ${store.errorText}` : ""}
          </pre>
        </div>
      </div>
    </div>
  );
}

// ─── Connect Page ────────────────────────────────────────────────────────────
function ConnectPage({ store }: { store: any }) {
  const [host, setHost] = useState(store.connection.host || "");
  const [port, setPort] = useState(store.connection.port || 22);
  const [username, setUsername] = useState(store.connection.username || "");
  const [password, setPassword] = useState("");
  const [configPath, setConfigPath] = useState(store.connection.configPath || "");
  const isConnecting = store.loadingTask === "connect";

  const handleConnect = async () => {
    if (!host.trim()) { store.notify(store.t.inputHost, "warning"); return; }
    if (!username.trim()) { store.notify(store.t.inputUser, "warning"); return; }
    if (!password) { store.notify(store.t.inputPass, "warning"); return; }
    
    await store.connect({ 
      host: host.trim(), 
      port: Number(port) || 22, 
      username: username.trim(), 
      password, 
      configPath: configPath.trim() || undefined 
    });
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter") handleConnect();
  };

  return (
    <div style={{ width: "100%", height: "100vh" }}>
      {/* Form panel — full width */}
      <div style={{
        width: "100%",
        display: "flex",
        flexDirection: "column",
        justifyContent: "center",
        padding: "48px 64px",
        backgroundColor: "#f8fafc",
        overflowY: "auto",
        position: "relative",
      }}>
        {/* Language Switcher in Login Page */}
        <div style={{ position: "absolute", top: 24, right: 32, display: "flex", gap: 8 }}>
          <button 
            className={`btn ${store.lang === "zh" ? "btn-primary" : "btn-outline"}`}
            style={{ height: 28, fontSize: 11, padding: "0 12px", minWidth: 60 }}
            onClick={() => store.setLang("zh")}
          >中文</button>
          <button 
            className={`btn ${store.lang === "en" ? "btn-primary" : "btn-outline"}`}
            style={{ height: 28, fontSize: 11, padding: "0 12px", minWidth: 60 }}
            onClick={() => store.setLang("en")}
          >EN</button>
        </div>

        <h2 style={{ margin: "0 0 8px 0", fontSize: 24, fontWeight: 700 }}>{store.t.loginTitle}</h2>
        <p style={{ margin: "0 0 32px 0", color: "var(--text-muted)", fontSize: 14 }}>{store.t.loginSubTitle}</p>

        <div style={{ display: "flex", flexDirection: "column", gap: 20 }} onKeyDown={handleKeyDown}>

          <div style={{ display: "flex", gap: 16 }}>
            <div style={{ flex: 1 }}>
              <label style={{ display: "block", marginBottom: 6, fontSize: 14, fontWeight: 500 }}>{store.t.host}</label>
              <input className="input" type="text" value={host} onChange={(e) => setHost(e.target.value)} placeholder="192.168.1.1" disabled={store.loading} />
            </div>
            <div style={{ width: 100 }}>
              <label style={{ display: "block", marginBottom: 6, fontSize: 14, fontWeight: 500 }}>{store.t.sshPort}</label>
              <input className="input" type="number" value={port} onChange={(e) => setPort(Number(e.target.value))} disabled={store.loading} />
            </div>
          </div>
          <div>
            <label style={{ display: "block", marginBottom: 6, fontSize: 14, fontWeight: 500 }}>{store.t.username}</label>
            <input className="input" type="text" value={username} onChange={(e) => setUsername(e.target.value)} placeholder="root" disabled={store.loading} />
          </div>
          <div>
            <label style={{ display: "block", marginBottom: 6, fontSize: 14, fontWeight: 500 }}>{store.t.password}</label>
            <input className="input" type="password" value={password} onChange={(e) => setPassword(e.target.value)} disabled={store.loading} />
          </div>
          <div>
            <label style={{ 
              display: "block", 
              marginBottom: 6, 
              fontSize: 14, 
              fontWeight: 500,
            }}>
              {store.t.configPath}
              <span style={{ color: "var(--text-muted)", fontWeight: 400 }}>{store.t.configPathHint}</span>
            </label>
            <input 
              className="input" 
              type="text" 
              value={configPath} 
              onChange={(e) => setConfigPath(e.target.value)} 
              placeholder="/etc/v2ray/config.json" 
              disabled={store.loading}
            />
          </div>
          <button
            className="btn btn-primary"
            style={{ marginTop: 8, height: 40, fontSize: 14, gap: 8, width: "fit-content", alignSelf: "flex-end", padding: "8px 16px" }}
            onClick={handleConnect}
            disabled={store.loading}
          >
            {isConnecting ? <><Spinner size={16} color="#fff" /> {store.t.connecting}</> : store.t.establishConn}
          </button>
        </div>
      </div>
    </div>
  );
}

// ─── Helpers ─────────────────────────────────────────────────────────────────
function getProtocolColor(protocol: string): string {
  switch (protocol.toLowerCase()) {
    case "vmess":  return "#2563eb";
    case "vless":  return "#059669";
    case "trojan": return "#7c3aed";
    case "ss":     return "#0891b2";
    case "ssr":    return "#db2777";
    default:       return "#64748b";
  }
}
