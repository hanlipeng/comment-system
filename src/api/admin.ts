import { invoke } from "@tauri-apps/api/core";

// --- 接口定义 ---
export interface Event {
  id: string;
  title: string;
  status: "active" | "closed" | "deleted";
  createdAt: number; // Unix Timestamp (Milliseconds)
  updatedAt: number; // Unix Timestamp (Milliseconds)
  winner?: Comment | null;
}

export interface Comment {
  id: string;
  eventId: string;
  nickname: string;
  content: string;
  phoneMasked: string;
  isWinner: boolean;
  createdAt: number; // Unix Timestamp (Milliseconds)
  updatedAt: number; // Unix Timestamp (Milliseconds)
}

export interface Settings {
  server_port: number;
  db_path: string;
  wifi_ssid?: string | null;
  wifi_password?: string | null;
  wifi_encryption?: string | null;
}

export interface ReloadSettingsChanged {
  server_port: boolean;
  db_path: boolean;
  wifi: boolean;
}

export interface ReloadSettingsResponse {
  settings: Settings;
  changed: ReloadSettingsChanged;
}

// --- API 实现 ---
const USE_MOCK = false;

// Mock Data
const mockEvents: Event[] = [];
const mockComments: Record<string, Comment[]> = {};

export const adminApi = {
  // --- 服务器控制 ---
  async startServer(): Promise<void> {
    if (USE_MOCK) return Promise.resolve();
    return invoke("start_server");
  },

  async stopServer(): Promise<void> {
    if (USE_MOCK) return Promise.resolve();
    return invoke("stop_server");
  },

  async isServerRunning(): Promise<boolean> {
     if (USE_MOCK) return Promise.resolve(false);
     return invoke("is_server_running");
  },

  async getNetworkIp(): Promise<string> {
    if (USE_MOCK) return Promise.resolve("127.0.0.1");
    return invoke("get_network_ip");
  },

  async reloadSettings(): Promise<ReloadSettingsResponse> {
    if (USE_MOCK) {
      return Promise.resolve({
        settings: { server_port: 3000, db_path: "./db/data.db" },
        changed: { server_port: false, db_path: false, wifi: false }
      });
    }
    return invoke("reload_settings");
  },

  // --- 活动管理 ---
  async updateEventStatus(eventId: string, status: "active" | "closed"): Promise<void> {
    if (USE_MOCK) return Promise.resolve();
    return invoke("admin_update_event_status", { eventId, status });
  },

  async deleteEvent(eventId: string): Promise<void> {
    if (USE_MOCK) return Promise.resolve();
    return invoke("admin_delete_event", { eventId });
  },

  async createEvent(title: string): Promise<string> {
    if (USE_MOCK) {
      const newId = (mockEvents.length + 1).toString();
      const now = Date.now();
      mockEvents.push({ 
          id: newId, 
          title, 
          status: "active",
          createdAt: now,
          updatedAt: now
      });
      mockComments[newId] = [];
      return Promise.resolve(newId);
    }
    // 注意后端命令名映射
    const ev: Event = await invoke("admin_create_event", { title });
    return ev.id;
  },

  async listEvents(): Promise<Event[]> {
    if (USE_MOCK) return Promise.resolve([...mockEvents]);
    return invoke("admin_list_events");
  },

  async getComments(eventId: string): Promise<Comment[]> {
    if (USE_MOCK) return Promise.resolve(mockComments[eventId] || []);
    return invoke("admin_get_comments", { eventId });
  },

  async drawWinner(eventId: string): Promise<Comment | null> {
    if (USE_MOCK) {
      const comments = mockComments[eventId] || [];
      if (comments.length === 0) return Promise.resolve(null);
      return Promise.resolve(comments[0]);
    }
    return invoke("admin_draw_winner", { eventId });
  }
};