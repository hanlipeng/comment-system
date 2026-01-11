<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { adminApi, type Event, type Comment } from '../api/admin';

const events = ref<Event[]>([]);
const selectedEvent = ref<Event | null>(null);
const comments = ref<Comment[]>([]);
const newEventTitle = ref('');
const loading = ref(false);

const loadEvents = async () => {
  events.value = await adminApi.listEvents();
};

const createEvent = async () => {
  if (!newEventTitle.value.trim()) return;
  await adminApi.createEvent(newEventTitle.value);
  newEventTitle.value = '';
  await loadEvents();
};

const selectEvent = async (event: Event) => {
  selectedEvent.value = event;
  comments.value = await adminApi.getComments(event.id);
};

const doDraw = async () => {
  if (!selectedEvent.value) return;
  loading.value = true;
  try {
    const winner = await adminApi.drawWinner(selectedEvent.value.id);
    if (winner) {
      alert(`恭喜中奖者：${winner.nickname} (${winner.phone})\n内容：${winner.content}`);
      await loadEvents(); // 刷新状态
      await selectEvent(selectedEvent.value); // 刷新当前视图
    } else {
      alert('没有留言，无法抽奖');
    }
  } catch (e) {
    alert('抽奖出错: ' + e);
  } finally {
    loading.value = false;
  }
};

onMounted(loadEvents);
</script>

<template>
  <div class="event-manager">
    <!-- 左侧列表 -->
    <div class="sidebar card">
      <div class="sidebar-header">
        <h3>活动项目</h3>
        <span class="count">{{ events.length }}</span>
      </div>
      
      <div class="create-box">
        <input v-model="newEventTitle" placeholder="输入活动名称..." @keyup.enter="createEvent" />
        <button @click="createEvent" class="btn-icon">＋</button>
      </div>

      <div class="event-list">
        <div 
          v-for="e in events" 
          :key="e.id" 
          @click="selectEvent(e)"
          class="event-item"
          :class="{ active: selectedEvent?.id === e.id }"
        >
          <div class="event-info">
            <span class="title">{{ e.title }}</span>
            <span class="id">#{{ e.id }}</span>
          </div>
          <span class="status-tag" :class="e.status">
            {{ e.status === 'active' ? '进行中' : '已结算' }}
          </span>
        </div>
      </div>
    </div>

    <!-- 右侧详情 -->
    <div class="main-content card" v-if="selectedEvent">
      <div class="header">
        <div class="title-wrap">
          <h2>{{ selectedEvent.title }}</h2>
          <div class="meta">
            <span class="tag">ID: {{ selectedEvent.id }}</span>
            <span class="tag" v-if="selectedEvent.status === 'ended'">已于 {{ new Date().toLocaleDateString() }} 结算</span>
          </div>
        </div>
        
        <button 
          v-if="selectedEvent.status === 'active'" 
          class="btn-draw" 
          @click="doDraw"
          :disabled="loading"
        >
          <span class="sparkle">✨</span>
          {{ loading ? '正在抽取...' : '执行抽奖' }}
        </button>
      </div>

      <transition name="slide-fade">
        <div v-if="selectedEvent.winner" class="winner-card">
          <div class="winner-badge">🏆 中奖公示</div>
          <div class="winner-content">
            <div class="avatar">{{ selectedEvent.winner.nickname[0] }}</div>
            <div class="info">
              <div class="nickname">{{ selectedEvent.winner.nickname }}</div>
              <div class="phone">{{ selectedEvent.winner.phone }}</div>
            </div>
            <div class="comment-text">“{{ selectedEvent.winner.content }}”</div>
          </div>
        </div>
      </transition>

      <div class="comments-section">
        <div class="section-header">
          <h3>留言互动 ({{ comments.length }})</h3>
        </div>
        
        <div class="table-container">
          <table class="comment-table">
            <thead>
              <tr>
                <th>用户</th>
                <th>留言内容</th>
                <th class="time-col">时间</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="c in comments" :key="c.id" :class="{ 'is-winner': selectedEvent.winner?.id === c.id }">
                <td class="user-cell">
                  <div class="name">{{ c.nickname }}</div>
                  <div class="phone">{{ c.phone }}</div>
                </td>
                <td class="content-cell">{{ c.content }}</td>
                <td class="time-cell">{{ new Date(c.created_at).toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'}) }}</td>
              </tr>
              <tr v-if="comments.length === 0">
                <td colspan="3" class="empty">
                  <div class="empty-icon">💬</div>
                  <p>尚无留言互动</p>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
    
    <div class="main-content card empty-state" v-else>
      <div class="empty-hero">
        <div class="hero-icon">📋</div>
        <h3>请选择活动</h3>
        <p>在左侧侧边栏选择一个活动来管理留言和抽奖</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.event-manager {
  display: flex;
  gap: 24px;
  height: 700px;
}

/* Sidebar Styling */
.sidebar {
  width: 300px;
  display: flex;
  flex-direction: column;
  background: #fff;
}

.sidebar-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px;
  border-bottom: 1px solid var(--border-color);
}

.sidebar-header h3 { margin: 0; font-size: 1rem; color: var(--text-main); }
.sidebar-header .count {
  background: #f1f5f9;
  padding: 2px 8px;
  border-radius: 12px;
  font-size: 0.75rem;
  font-weight: 700;
  color: var(--text-muted);
}

.create-box {
  padding: 16px;
  display: flex;
  gap: 8px;
}

.create-box input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  font-size: 0.875rem;
}

.btn-icon {
  width: 36px;
  height: 36px;
  background: var(--primary-color);
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.2rem;
}

.event-list {
  flex: 1;
  overflow-y: auto;
  padding: 8px;
}

.event-item {
  padding: 12px;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 4px;
}

.event-item:hover { background: #f8fafc; }
.event-item.active { background: #eff6ff; }

.event-info { display: flex; flex-direction: column; }
.event-info .title { font-size: 0.9rem; font-weight: 600; color: var(--text-main); }
.event-info .id { font-size: 0.75rem; color: var(--text-muted); }

.status-tag {
  font-size: 0.7rem;
  font-weight: 700;
  padding: 2px 8px;
  border-radius: 4px;
  text-transform: uppercase;
}
.status-tag.active { background: #dcfce7; color: #166534; }
.status-tag.ended { background: #f1f5f9; color: #475569; }

/* Main Content Styling */
.main-content { flex: 1; overflow-y: auto; display: flex; flex-direction: column; }

.header {
  padding: 24px;
  border-bottom: 1px solid var(--border-color);
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
}

.title-wrap h2 { margin: 0; font-size: 1.5rem; color: var(--text-main); }
.meta { display: flex; gap: 8px; margin-top: 8px; }
.tag { font-size: 0.75rem; color: var(--text-muted); background: #f1f5f9; padding: 2px 8px; border-radius: 4px; }

.btn-draw {
  background: linear-gradient(135deg, #f59e0b, #d97706);
  color: white;
  border: none;
  padding: 12px 24px;
  border-radius: 8px;
  font-weight: 700;
  cursor: pointer;
  box-shadow: 0 4px 12px rgba(217, 119, 6, 0.3);
  transition: all 0.2s;
  display: flex;
  align-items: center;
  gap: 8px;
}

.btn-draw:hover { transform: translateY(-2px); box-shadow: 0 6px 16px rgba(217, 119, 6, 0.4); }
.btn-draw:active { transform: translateY(0); }

.winner-card {
  margin: 24px;
  background: linear-gradient(to right, #fffbeb, #fef3c7);
  border: 1px solid #fde68a;
  border-radius: 12px;
  padding: 20px;
  position: relative;
  overflow: hidden;
}

.winner-badge {
  position: absolute;
  top: 12px;
  right: -20px;
  background: #f59e0b;
  color: white;
  padding: 4px 24px;
  transform: rotate(45deg);
  font-size: 0.75rem;
  font-weight: 800;
}

.winner-content { display: flex; align-items: center; gap: 20px; }
.avatar {
  width: 48px;
  height: 48px;
  background: #f59e0b;
  color: white;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.5rem;
  font-weight: 700;
}

.winner-content .nickname { font-size: 1.1rem; font-weight: 700; color: #92400e; }
.winner-content .phone { font-size: 0.85rem; color: #b45309; }
.winner-content .comment-text { font-style: italic; color: #92400e; margin-left: auto; max-width: 300px; text-align: right; }

.comments-section { padding: 0 24px 24px; flex: 1; }
.section-header { margin-bottom: 16px; }

.comment-table { width: 100%; border-collapse: collapse; }
.comment-table th { text-align: left; padding: 12px; font-size: 0.75rem; text-transform: uppercase; color: var(--text-muted); border-bottom: 1px solid var(--border-color); }
.comment-table td { padding: 16px 12px; border-bottom: 1px solid #f1f5f9; vertical-align: middle; }

.user-cell .name { font-weight: 600; font-size: 0.9rem; }
.user-cell .phone { font-size: 0.75rem; color: var(--text-muted); }
.content-cell { font-size: 0.9rem; color: #475569; }
.time-cell { font-size: 0.75rem; color: var(--text-muted); text-align: right; }

tr.is-winner { background: #fffbeb; }

.empty { text-align: center; padding: 60px 0; color: var(--text-muted); }
.empty-icon { font-size: 3rem; margin-bottom: 12px; }

.empty-state { align-items: center; justify-content: center; text-align: center; }
.hero-icon { font-size: 4rem; margin-bottom: 16px; opacity: 0.5; }

/* Transitions */
.slide-fade-enter-active { transition: all 0.3s ease-out; }
.slide-fade-enter-from { transform: translateY(-20px); opacity: 0; }
</style>
