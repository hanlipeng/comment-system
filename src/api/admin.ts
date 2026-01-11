import { invoke } from "@tauri-apps/api/core";

// --- 接口定义 ---
export interface Event {
  id: string;
  title: string;
  status: "active" | "ended";
  winner?: Comment | null;
}

export interface Comment {
  id: string;
  event_id: string;
  nickname: string;
  content: string;
  phone: string;
  created_at: number;
}

// --- 模拟数据 ---
let mockServerRunning = false;
let mockEvents: Event[] = [
  { id: "1", title: "公司年会抽奖", status: "active" },
  { 
    id: "2", 
    title: "上季度优秀员工评选", 
    status: "ended",
    winner: { 
      id: "205", 
      event_id: "2", 
      nickname: "王五", 
      content: "感谢大家的认可，我会继续努力！", 
      phone: "136****8888", 
      created_at: Date.now() - 86400000 
    }
  }
];
let mockComments: Record<string, Comment[]> = {
  "1": [
    { id: "101", event_id: "1", nickname: "张三", content: "中大奖！", phone: "138****1111", created_at: Date.now() },
    { id: "102", event_id: "1", nickname: "李四", content: "祝大家新年快乐", phone: "139****2222", created_at: Date.now() }
  ],
  "2": [
     { id: "201", event_id: "2", nickname: "赵六", content: "我也想中奖", phone: "137****3333", created_at: Date.now() - 90000000 },
     { id: "205", event_id: "2", nickname: "王五", content: "感谢大家的认可，我会继续努力！", phone: "136****8888", created_at: Date.now() - 86400000 }
  ]
};

// --- API 实现 ---
const USE_MOCK = true; // 当后端 Rust 命令准备好后，将其改为 false

export const adminApi = {
  async startServer(port: number): Promise<void> {
    if (USE_MOCK) {
      console.log(`[Mock] 正在端口 ${port} 启动服务器...`);
      mockServerRunning = true;
      return Promise.resolve();
    }
    return invoke("start_server", { port });
  },

  async stopServer(): Promise<void> {
    if (USE_MOCK) {
      console.log("[Mock] 正在停止服务器...");
      mockServerRunning = false;
      return Promise.resolve();
    }
    return invoke("stop_server");
  },

  async isServerRunning(): Promise<boolean> {
     if (USE_MOCK) return Promise.resolve(mockServerRunning);
     // 假设后端有一个状态查询，或者前端自己维护
     return Promise.resolve(mockServerRunning);
  },

  async createEvent(title: string): Promise<string> {
    if (USE_MOCK) {
      const newId = (mockEvents.length + 1).toString();
      const newEvent: Event = { id: newId, title, status: "active" };
      mockEvents.push(newEvent);
      mockComments[newId] = [];
      return Promise.resolve(newId);
    }
    return invoke("create_event", { title });
  },

  async listEvents(): Promise<Event[]> {
    if (USE_MOCK) {
      return Promise.resolve([...mockEvents]);
    }
    return invoke("list_events");
  },

  async getComments(eventId: string): Promise<Comment[]> {
    if (USE_MOCK) {
      return Promise.resolve(mockComments[eventId] || []);
    }
    return invoke("get_comments", { event_id: eventId });
  },

  async drawWinner(eventId: string): Promise<Comment | null> {
    if (USE_MOCK) {
      const comments = mockComments[eventId] || [];
      if (comments.length === 0) return Promise.resolve(null);
      const winner = comments[Math.floor(Math.random() * comments.length)];
      
      const event = mockEvents.find(e => e.id === eventId);
      if (event) {
          event.winner = winner;
          event.status = "ended";
      }
      return Promise.resolve(winner);
    }
    return invoke("draw_winner", { event_id: eventId });
  }
};
