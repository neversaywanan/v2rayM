import { invoke } from "@tauri-apps/api/core";
import type {
  ApplyPayload,
  ApplyResult,
  ConnectPayload,
  FetchResult,
  SubscriptionParseResult,
  SwitchOutboundPayload,
  UpdatePortPayload,
} from "../types/api";

export const api = {
  connectAndFetchConfig(payload: ConnectPayload): Promise<FetchResult> {
    return invoke("connect_and_fetch_config", { payload });
  },

  fetchSubscription(url: string): Promise<SubscriptionParseResult> {
    return invoke("fetch_subscription", { url });
  },

  applyRemoteConfig(payload: ApplyPayload): Promise<ApplyResult> {
    return invoke("apply_remote_config", { payload });
  },

  rollbackRemoteConfig(payload: ConnectPayload): Promise<ApplyResult> {
    return invoke("rollback_remote_config", { payload });
  },

  testProxyConnection(payload: ConnectPayload): Promise<string> {
    return invoke("test_proxy_connection", { payload });
  },

  switchActiveOutbound(payload: SwitchOutboundPayload): Promise<void> {
    return invoke("switch_active_outbound", { payload });
  },

  updateInboundPort(payload: UpdatePortPayload): Promise<void> {
    return invoke("update_inbound_port", { payload });
  },
};
