// src/api/client.ts

export interface Comment {
  nickname: string;
  content: string;
  phone: string;
}

export interface Winner {
  nickname: string;
  content: string;
  phoneMasked: string;
}

export interface PublicComment {
    nickname: string;
    content: string;
    phoneMasked: string;
    createdAt: number;
}

export interface EventData {
  id: string;
  title: string;
  status: 'active' | 'closed';
  winner: Winner | null;
}

export interface WifiConfig {
    ssid: string;
    encryption: string; // WPA, WEP, nopass
    password?: string;
}

// Environment Configuration
const API_BASE_URL = "/api/v1";
// Use Mock during development (npm run dev)
const USE_MOCK = import.meta.env.DEV;

export class ApiError extends Error {
  status: number;
  constructor(message: string, status: number) {
    super(message);
    this.status = status;
    this.name = 'ApiError';
  }
}

// ==========================================
// Mock Data & Helpers (Restored)
// ==========================================
const delay = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

let mockEvent: EventData = {
  id: '1',
  title: '🎉 2026 新年幸运抽奖活动 (Mock)',
  status: 'active',
  winner: null
};

const mockWifi: WifiConfig = {
    ssid: "Gemini_Guest_2026",
    encryption: "WPA",
    password: "password123"
};

// Generate some mock comments
const mockComments: PublicComment[] = Array.from({ length: 25 }, (_, i) => ({
    nickname: `用户${1000 + i}`,
    content: `这是第 ${i + 1} 条许愿留言，希望能中大奖！ #2026`,
    phoneMasked: `138****${String(1000 + i).slice(-4)}`,
    createdAt: Date.now() - i * 60000
}));

// ==========================================
// API Implementation
// ==========================================

export const api = {
  // 3.1 获取当前活跃活动
  async getActiveEvent(): Promise<EventData | null> {
    if (USE_MOCK) {
        await delay(600);
        console.log(`[Mock API] GET /events/active`, mockEvent);
        return { ...mockEvent };
    }

    const response = await fetch(`${API_BASE_URL}/events/active`);
    if (!response.ok) {
       throw new ApiError(`Failed to fetch active event: ${response.statusText}`, response.status);
    }
    const data = await response.json();
    return data as EventData | null;
  },

  // 3.2 获取指定活动详情
  async getEvent(id: string): Promise<EventData> {
    if (USE_MOCK) {
        await delay(400);
        console.log(`[Mock API] GET /events/${id}`, mockEvent);
        if (mockEvent.id !== id) throw new ApiError("Event not found", 404);
        return { ...mockEvent };
    }

    const response = await fetch(`${API_BASE_URL}/events/${id}`);
     if (!response.ok) {
       if (response.status === 404) {
           throw new ApiError("Event not found", 404);
       }
       throw new ApiError(`Failed to fetch event: ${response.statusText}`, response.status);
    }
    const data = await response.json();
    return data as EventData;
  },

  // 3.3 提交留言
  async postComment(eventId: string, data: Comment): Promise<void> {
    // TODO: Implement server-side check for 'one phone one submission' or 'one device one submission'
    // This requires backend support to track submission per phone/IP/device_fingerprint.
    // Current implementation relies on client-side cookies which can be bypassed.
    
    if (USE_MOCK) {
        await delay(1000);
        console.log(`[Mock API] POST /events/${eventId}/comments`, data);
        
        if (!data.content || !data.nickname || !data.phone) {
          throw new Error("请填写所有必填字段");
        }
        if (!/^1[3-9]\d{9}$/.test(data.phone)) {
          throw new Error("请输入正确的手机号");
        }
        // Add to mock comments
        mockComments.unshift({
            nickname: data.nickname,
            content: data.content,
            phoneMasked: data.phone.replace(/(\d{3})\d{4}(\d{4})/, '$1****$2'),
            createdAt: Date.now()
        });
        return;
    }

    const response = await fetch(`${API_BASE_URL}/events/${eventId}/comments`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(data)
    });

    if (!response.ok) {
        let errorMsg = response.statusText;
        try {
            const errorText = await response.text();
            if (errorText) errorMsg = errorText;
        } catch (e) {
            // ignore
        }
        throw new Error(errorMsg || `Failed to post comment: ${response.status}`);
    }
  },

  // 3.4 获取最新留言列表
  async getComments(eventId: string): Promise<PublicComment[]> {
      if (USE_MOCK) {
          await delay(300);
          console.log(`[Mock API] GET /events/${eventId}/comments`);
          // Return top 20
          return [...mockComments].slice(0, 20);
      }

      const response = await fetch(`${API_BASE_URL}/events/${eventId}/comments`);
      if (!response.ok) {
          throw new Error(`Failed to fetch comments: ${response.statusText}`);
      }
      const data = await response.json();
      return data as PublicComment[];
  },

  // 3.5 获取 WiFi 配置
  async getWifiConfig(): Promise<WifiConfig | null> {
      if (USE_MOCK) {
          await delay(200);
          console.log(`[Mock API] GET /wifi`, mockWifi);
          return { ...mockWifi };
      }

      const response = await fetch(`${API_BASE_URL}/wifi`);
      if (!response.ok) {
          if (response.status === 404) return null;
          throw new Error(`Failed to fetch wifi config: ${response.statusText}`);
      }
      const data = await response.json();
      return data as WifiConfig;
  },

  // (Dev Only) 模拟管理员开奖
  async _dev_triggerDraw() {
    if (USE_MOCK) {
        mockEvent.status = 'closed';
        mockEvent.winner = {
          nickname: '幸运的 Gemini 用户',
          content: '这个评论系统做得很棒！手机号校验也生效了。',
          phoneMasked: '138****8888'
        };
        return;
    }
    console.warn("Dev methods are not available in integrated mode.");
  },

  // (Dev Only) 重置活动
  async _dev_reset() {
    if (USE_MOCK) {
        mockEvent.status = 'active';
        mockEvent.winner = null;
        return;
    }
    console.warn("Dev methods are not available in integrated mode.");
  }
};
