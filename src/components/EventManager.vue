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
      alert(`恭喜中奖者：${winner.nickname} (${winner.phoneMasked})\n内容：${winner.content}`);
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

const updateStatus = async (status: "active" | "closed") => {
  if (!selectedEvent.value) return;
  const msg = status === "closed" ? "确定要结束该活动吗？结束以后用户将无法提交留言。" : "确定要重启该活动吗？";
  if (!confirm(msg)) return;

  try {
    await adminApi.updateEventStatus(selectedEvent.value.id, status);
    await loadEvents();
    // 刷新本地选中的活动状态
    if (selectedEvent.value) {
      selectedEvent.value.status = status;
    }
  } catch (e) {
    alert('更新状态失败: ' + e);
  }
};

const doDelete = async () => {
  if (!selectedEvent.value) return;
  if (!confirm(`⚠️ 危险操作：确定要删除活动 "${selectedEvent.value.title}" 吗？\n删除后将无法恢复。`)) return;

  try {
    await adminApi.deleteEvent(selectedEvent.value.id);
    selectedEvent.value = null; // 清空选中
    await loadEvents(); // 刷新列表
  } catch (e) {
    alert('删除失败: ' + e);
  }
};

const formatDate = (ts: number) => {
  if (!ts) return '-';
  const date = new Date(ts);
  const Y = date.getFullYear();
  const M = String(date.getMonth() + 1).padStart(2, '0');
  const D = String(date.getDate()).padStart(2, '0');
  const h = String(date.getHours()).padStart(2, '0');
  const m = String(date.getMinutes()).padStart(2, '0');
  return `${Y}/${M}/${D} ${h}:${m}`;
};

onMounted(loadEvents);
</script>

<template>
  <div class="event-manager">
    <!-- 左侧列表 -->
    <div class="sidebar card">
      <div class="sidebar-header">
        <div class="title-with-icon">
          <span class="sidebar-icon">📑</span>
          <h3>活动项目</h3>
        </div>
        <span class="count">{{ events.length }}</span>
      </div>
      
      <div class="create-box">
        <div class="input-wrapper">
          <input v-model="newEventTitle" placeholder="策划新活动..." @keyup.enter="createEvent" />
          <button @click="createEvent" class="btn-add">＋</button>
        </div>
      </div>

      <div class="event-list">
        <div 
          v-for="e in events" 
          :key="e.id" 
          @click="selectEvent(e)"
          class="event-item"
          :class="{ active: selectedEvent?.id === e.id }"
        >
          <div class="event-indicator" v-if="selectedEvent?.id === e.id"></div>
          <div class="event-info">
            <span class="title">{{ e.title }}</span>
            <div class="meta-row">
              <span class="id-badge">ID: {{ e.id.slice(0, 4) }}</span>
              <span v-if="e.status === 'active'" class="time created">
                🕒 {{ formatDate(e.createdAt) }}
              </span>
            </div>
          </div>
          <div class="status-dot" :class="e.status"></div>
        </div>
      </div>
    </div>

    <!-- 右侧详情 -->
    <div class="main-content card" v-if="selectedEvent">
      <div class="detail-header">
        <div class="title-wrap">
          <div class="title-meta">
            <span class="status-pill" :class="selectedEvent.status">
              {{ selectedEvent.status === 'active' ? '进行中' : '已结束' }}
            </span>
            <span class="id-label">#{{ selectedEvent.id }}</span>
          </div>
          <h2>{{ selectedEvent.title }}</h2>
        </div>
        
        <div class="actions">
          <button 
            class="btn-action danger outline"
            @click="doDelete"
            title="删除活动"
          >
            🗑️
          </button>
          <div class="action-divider"></div>
          <button 
            v-if="selectedEvent.status === 'active'" 
            class="btn-action secondary" 
            @click="updateStatus('closed')"
          >
            结束活动
          </button>
          <button 
            v-if="selectedEvent.status === 'closed'" 
            class="btn-action secondary" 
            @click="updateStatus('active')"
          >
            重新开启
          </button>
          <button 
            v-if="selectedEvent.status === 'active'" 
            class="btn-action primary draw" 
            @click="doDraw"
            :disabled="loading"
          >
            <span class="sparkle">✨</span>
            {{ loading ? '正在抽取...' : '执行抽奖' }}
          </button>
        </div>
      </div>

      <div class="detail-body">
        <transition name="winner-reveal">
          <div v-if="selectedEvent.winner" class="winner-hero">
            <div class="winner-glass">
              <div class="winner-badge">
                <span class="badge-icon">🏆</span>
                幸运中奖者
              </div>
              <div class="winner-profile">
                <div class="winner-avatar">{{ selectedEvent.winner.nickname[0] }}</div>
                <div class="winner-identity">
                  <div class="name">{{ selectedEvent.winner.nickname }}</div>
                  <div class="phone">{{ selectedEvent.winner.phoneMasked }}</div>
                </div>
                <div class="winner-comment">
                  <span class="quote-icon">“</span>
                  {{ selectedEvent.winner.content }}
                  <span class="quote-icon">”</span>
                </div>
              </div>
            </div>
          </div>
        </transition>

        <div class="comments-section">
          <div class="section-header">
            <h3>实时互动流 <span class="comment-count">{{ comments.length }}</span></h3>
          </div>
          
          <div class="table-container">
            <table class="comment-table">
              <thead>
                <tr>
                  <th>用户标识</th>
                  <th>留言互动内容</th>
                  <th class="time-col">时间</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="c in comments" :key="c.id" :class="{ 'is-winner': selectedEvent.winner?.id === c.id }">
                  <td class="user-cell">
                    <div class="user-item">
                      <div class="mini-avatar">{{ c.nickname[0] }}</div>
                      <div>
                        <div class="name">{{ c.nickname }}</div>
                        <div class="phone">{{ c.phoneMasked }}</div>
                      </div>
                    </div>
                  </td>
                  <td class="content-cell">
                    <div class="content-bubble">{{ c.content }}</div>
                  </td>
                  <td class="time-cell">{{ new Date(c.createdAt).toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'}) }}</td>
                </tr>
              </tbody>
            </table>
            
            <div v-if="comments.length === 0" class="empty-feed">
              <div class="empty-anim">💬</div>
              <p>等待第一条留言...</p>
            </div>
          </div>
        </div>
      </div>
    </div>
    
    <div class="main-content card empty-state" v-else>
      <div class="empty-hero">
        <div class="hero-graphic">
          <div class="dot-grid"></div>
          <div class="hero-icon">📋</div>
        </div>
        <h3>就绪</h3>
        <p>请在左侧选择一个活动项目开始管理</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.event-manager {
  display: flex;
  gap: 32px;
  height: 760px;
}

/* Sidebar Refinement */
.sidebar {
  width: 320px;
  display: flex;
  flex-direction: column;
  background: rgba(255, 255, 255, 0.6);
}

.sidebar-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 24px;
  border-bottom: 1px solid var(--border-color);
}

.title-with-icon {
  display: flex;
  align-items: center;
  gap: 10px;
}

.sidebar-icon { font-size: 1.25rem; }
.sidebar-header h3 { margin: 0; font-size: 1rem; font-weight: 800; color: var(--text-main); }

.count {
  background: var(--primary-color);
  color: white;
  padding: 2px 10px;
  border-radius: 20px;
  font-size: 0.7rem;
  font-weight: 800;
}

.create-box {
  padding: 20px;
}

.input-wrapper {
  display: flex;
  background: white;
  border: 1px solid var(--border-color);
  border-radius: 10px;
  padding: 4px;
  transition: border-color 0.2s;
}

.input-wrapper:focus-within { border-color: var(--accent-blue); }

.input-wrapper input {
  flex: 1;
  border: none;
  padding: 8px 12px;
  font-size: 0.85rem;
  outline: none;
  background: transparent;
}

.btn-add {
  width: 32px;
  height: 32px;
  background: var(--primary-color);
  color: white;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-weight: 800;
}

.event-list {
  flex: 1;
  overflow-y: auto;
  padding: 12px;
}

.event-item {
  padding: 16px;
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
  position: relative;
  background: transparent;
}

.event-item:hover { background: rgba(255, 255, 255, 0.8); }
.event-item.active { background: white; box-shadow: var(--shadow-sm); }

.event-indicator {
  position: absolute;
  left: 0;
  top: 16px;
  bottom: 16px;
  width: 4px;
  background: var(--accent-blue);
  border-radius: 0 4px 4px 0;
}

.event-info .title { font-size: 0.95rem; font-weight: 700; color: var(--text-main); display: block; margin-bottom: 4px; }
.id-badge { font-family: "Geist Mono", monospace; font-size: 0.7rem; color: var(--text-muted); background: #f1f5f9; padding: 2px 6px; border-radius: 4px; }
.meta-row { display: flex; align-items: center; gap: 8px; margin-top: 4px; }
.time { font-size: 0.7rem; color: var(--text-muted); }

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #cbd5e1;
}
.status-dot.active { background: var(--success-color); box-shadow: 0 0 8px var(--success-color); }

/* Detail View Refinement */
.main-content { flex: 1; display: flex; flex-direction: column; overflow: hidden; }

.detail-header {
  padding: 32px;
  border-bottom: 1px solid var(--border-color);
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
}

.title-meta { display: flex; gap: 12px; align-items: center; margin-bottom: 8px; }
.status-pill {
  font-size: 0.65rem;
  font-weight: 800;
  padding: 4px 10px;
  border-radius: 20px;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}
.status-pill.active { background: rgba(16, 185, 129, 0.1); color: var(--success-color); }
.status-pill.closed { background: #f1f5f9; color: var(--text-muted); }
.id-label { font-family: "Geist Mono", monospace; font-size: 0.75rem; color: var(--text-muted); }

.detail-header h2 { margin: 0; font-size: 1.75rem; font-weight: 800; letter-spacing: -0.03em; }

.actions { display: flex; gap: 12px; align-items: center; }

.btn-action {
  height: 42px;
  padding: 0 18px;
  border-radius: 10px;
  font-weight: 700;
  font-size: 0.85rem;
  cursor: pointer;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  gap: 8px;
  border: 1px solid transparent;
}

.btn-action.primary { background: var(--primary-color); color: white; }
.btn-action.secondary { background: white; border-color: var(--border-color); color: var(--text-main); }
.btn-action.secondary:hover { background: #f8fafc; }
.btn-action.danger.outline { background: transparent; border-color: transparent; color: var(--text-muted); padding: 0 12px; }
.btn-action.danger.outline:hover { background: #fef2f2; color: var(--danger-color); border-color: #fee2e2; }

.btn-action.draw {
  background: linear-gradient(135deg, #f59e0b, #d97706);
  box-shadow: 0 4px 12px rgba(217, 119, 6, 0.3);
}
.btn-action.draw:hover { transform: translateY(-2px); box-shadow: 0 6px 16px rgba(217, 119, 6, 0.4); }

.action-divider { width: 1px; height: 24px; background: var(--border-color); margin: 0 4px; }

.detail-body { flex: 1; overflow-y: auto; }

/* Winner Hero */
.winner-hero { padding: 32px 32px 0; }
.winner-glass {
  background: linear-gradient(135deg, #fffbeb, #fef3c7);
  border: 1px solid #fde68a;
  border-radius: 20px;
  padding: 24px;
  position: relative;
  overflow: hidden;
  box-shadow: 0 10px 25px -5px rgba(245, 158, 11, 0.15);
}

.winner-badge {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  background: #f59e0b;
  color: white;
  font-size: 0.7rem;
  font-weight: 800;
  padding: 4px 12px;
  border-radius: 20px;
  margin-bottom: 20px;
  text-transform: uppercase;
}

.winner-profile { display: flex; align-items: center; gap: 24px; }
.winner-avatar {
  width: 64px;
  height: 64px;
  background: #f59e0b;
  color: white;
  border-radius: 18px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 2rem;
  font-weight: 800;
}

.winner-identity .name { font-size: 1.25rem; font-weight: 800; color: #92400e; }
.winner-identity .phone { font-size: 0.85rem; color: #b45309; font-family: "Geist Mono", monospace; }

.winner-comment {
  flex: 1;
  font-style: italic;
  font-size: 1rem;
  color: #92400e;
  text-align: right;
  padding-left: 40px;
  position: relative;
}
.quote-icon { font-size: 1.5rem; opacity: 0.3; color: #f59e0b; font-family: serif; }

/* Comments Section */
.comments-section { padding: 32px; }
.section-header { margin-bottom: 24px; display: flex; align-items: center; justify-content: space-between; }
.section-header h3 { margin: 0; font-size: 1.1rem; font-weight: 800; color: var(--text-main); }
.comment-count { font-size: 0.85rem; color: var(--text-muted); font-weight: 400; margin-left: 8px; }

.comment-table { width: 100%; border-collapse: separate; border-spacing: 0 8px; }
.comment-table th { text-align: left; padding: 12px 16px; font-size: 0.7rem; font-weight: 800; text-transform: uppercase; color: var(--text-muted); letter-spacing: 0.05em; }

.comment-table td { padding: 16px; background: rgba(255, 255, 255, 0.4); vertical-align: middle; }
.comment-table tr td:first-child { border-radius: 12px 0 0 12px; }
.comment-table tr td:last-child { border-radius: 0 12px 12px 0; }

.user-item { display: flex; align-items: center; gap: 12px; }
.mini-avatar { width: 32px; height: 32px; background: #e2e8f0; border-radius: 8px; display: flex; align-items: center; justify-content: center; font-size: 0.8rem; font-weight: 700; color: #64748b; }

.user-item .name { font-weight: 700; font-size: 0.9rem; }
.user-item .phone { font-size: 0.75rem; color: var(--text-muted); font-family: "Geist Mono", monospace; }

.content-bubble {
  background: white;
  padding: 10px 16px;
  border-radius: 10px;
  font-size: 0.9rem;
  color: #475569;
  display: inline-block;
  max-width: 400px;
  border: 1px solid #f1f5f9;
}

.is-winner td { background: #fffbeb !important; }
.is-winner .content-bubble { border-color: #fde68a; }

.time-cell { font-size: 0.75rem; color: var(--text-muted); font-family: "Geist Mono", monospace; text-align: right; }

.empty-feed { text-align: center; padding: 80px 0; color: var(--text-muted); }
.empty-anim { font-size: 3rem; margin-bottom: 16px; opacity: 0.5; animation: float 3s ease-in-out infinite; }

@keyframes float {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-10px); }
}

/* Empty State */
.empty-state { display: flex; align-items: center; justify-content: center; }
.hero-graphic { position: relative; width: 120px; height: 120px; margin: 0 auto 32px; display: flex; align-items: center; justify-content: center; }
.dot-grid { position: absolute; width: 100%; height: 100%; background-image: radial-gradient(#e2e8f0 1px, transparent 1px); background-size: 12px 12px; opacity: 0.5; }
.hero-icon { font-size: 4rem; position: relative; z-index: 1; filter: drop-shadow(0 10px 15px rgba(0,0,0,0.1)); }
.empty-hero h3 { font-size: 1.5rem; font-weight: 800; margin-bottom: 8px; }
.empty-hero p { color: var(--text-muted); font-size: 0.95rem; }

/* Animations */
.winner-reveal-enter-active { transition: all 0.6s cubic-bezier(0.16, 1, 0.3, 1); }
.winner-reveal-enter-from { transform: scale(0.9) translateY(20px); opacity: 0; }
</style>
