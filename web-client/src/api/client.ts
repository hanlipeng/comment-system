// src/api/client.ts

export interface Comment {
  nickname: string;
  content: string;
  phone: string;
}

export interface Winner {
  nickname: string;
  content: string;
  phone_masked: string;
}

export interface EventData {
  id: string;
  title: string;
  status: 'active' | 'closed';
  winner: Winner | null;
}

// src/api/client.ts

export interface Comment {
  nickname: string;
  content: string;
  phone: string;
}

export interface Winner {
  nickname: string;
  content: string;
  phone_masked: string;
}

export interface EventData {
  id: string;
  title: string;
  status: 'active' | 'closed';
  winner: Winner | null;
}

// Environment Configuration
const API_BASE_URL = "http://localhost:3000/api/v1";
// Use Mock during development (npm run dev)
const USE_MOCK = import.meta.env.DEV;

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

// ==========================================
// API Implementation
// ==========================================

export const api = {
  // 3.1 获取当前活跃活动
  async getActiveEvent(): Promise<EventData> {
    if (USE_MOCK) {
        await delay(600);
        console.log(`[Mock API] GET /events/active`, mockEvent);
        return { ...mockEvent };
    }

    const response = await fetch(`${API_BASE_URL}/events/active`);
    if (!response.ok) {
       if (response.status === 404) {
           throw new Error("No active event found");
       }
       throw new Error(`Failed to fetch active event: ${response.statusText}`);
    }
    const data = await response.json();
    return data as EventData;
  },

  // 3.2 获取指定活动详情
  async getEvent(id: string): Promise<EventData> {
    if (USE_MOCK) {
        await delay(400);
        console.log(`[Mock API] GET /events/${id}`, mockEvent);
        if (mockEvent.id !== id) throw new Error("Event not found");
        return { ...mockEvent };
    }

    const response = await fetch(`${API_BASE_URL}/events/${id}`);
     if (!response.ok) {
       if (response.status === 404) {
           throw new Error("Event not found");
       }
       throw new Error(`Failed to fetch event: ${response.statusText}`);
    }
    const data = await response.json();
    return data as EventData;
  },

  // 3.3 提交留言
  async postComment(eventId: string, data: Comment): Promise<void> {
    if (USE_MOCK) {
        await delay(1000);
        console.log(`[Mock API] POST /events/${eventId}/comments`, data);
        
        if (!data.content || !data.nickname || !data.phone) {
          throw new Error("请填写所有必填字段");
        }
        if (!/^1[3-9]\d{9}$/.test(data.phone)) {
          throw new Error("请输入正确的手机号");
        }
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

  // (Dev Only) 模拟管理员开奖
  async _dev_triggerDraw() {
    if (USE_MOCK) {
        mockEvent.status = 'closed';
        mockEvent.winner = {
          nickname: '幸运的 Gemini 用户',
          content: '这个评论系统做得很棒！手机号校验也生效了。',
          phone_masked: '138****8888'
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
