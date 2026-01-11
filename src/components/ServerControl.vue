<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { adminApi } from '../api/admin';

const port = ref(8080);
const isRunning = ref(false);
const loading = ref(false);

const toggleServer = async () => {
  loading.value = true;
  try {
    if (isRunning.value) {
      await adminApi.stopServer();
      isRunning.value = false;
    } else {
      await adminApi.startServer(port.value);
      isRunning.value = true;
    }
  } catch (e) {
    alert('操作失败: ' + e);
  } finally {
    loading.value = false;
  }
};

onMounted(async () => {
  isRunning.value = await adminApi.isServerRunning();
});
</script>

<template>
  <div class="card server-control">
    <div class="header">
      <div class="title-area">
        <span class="icon">🌐</span>
        <h3>HTTP 服务配置</h3>
      </div>
      <div class="status-indicator" :class="{ 'online': isRunning }">
        <span class="dot"></span>
        {{ isRunning ? '服务运行中' : '服务已停止' }}
      </div>
    </div>
    
    <div class="control-body">
      <div class="input-group">
        <label>监听端口</label>
        <div class="input-wrapper">
          <input type="number" v-model="port" :disabled="isRunning" placeholder="8080" />
          <span class="prefix">:</span>
        </div>
      </div>

      <button 
        @click="toggleServer" 
        :class="{ 'btn-stop': isRunning, 'btn-start': !isRunning }"
        :disabled="loading"
      >
        <template v-if="loading">
          <span class="spinner"></span> 
        </template>
        <template v-else>
          {{ isRunning ? '停止服务' : '开启服务' }}
        </template>
      </button>
    </div>
  </div>
</template>

<style scoped>
.server-control {
  padding: 20px;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.title-area {
  display: flex;
  align-items: center;
  gap: 8px;
}

.title-area h3 {
  margin: 0;
  font-size: 1.1rem;
  color: var(--text-main);
}

.icon { font-size: 1.2rem; }

.status-indicator {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 0.85rem;
  font-weight: 600;
  color: var(--text-muted);
  background: #f1f5f9;
  padding: 4px 12px;
  border-radius: 20px;
}

.status-indicator.online {
  color: var(--success-color);
  background: #ecfdf5;
}

.status-indicator .dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #94a3b8;
}

.status-indicator.online .dot {
  background: var(--success-color);
  box-shadow: 0 0 8px var(--success-color);
}

.control-body {
  display: flex;
  align-items: flex-end;
  gap: 24px;
}

.input-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.input-group label {
  font-size: 0.75rem;
  font-weight: 700;
  text-transform: uppercase;
  color: var(--text-muted);
  letter-spacing: 0.05em;
}

.input-wrapper {
  position: relative;
  display: flex;
  align-items: center;
}

input[type="number"] {
  width: 120px;
  padding: 10px 12px;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  font-size: 1rem;
  background: #f8fafc;
  transition: all 0.2s;
}

input:focus {
  outline: none;
  border-color: var(--primary-color);
  background: #fff;
  box-shadow: 0 0 0 3px rgba(37, 99, 235, 0.1);
}

button {
  height: 42px;
  padding: 0 24px;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-weight: 600;
  font-size: 0.95rem;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
}

.btn-start {
  background: var(--primary-color);
  color: white;
}

.btn-start:hover { background: var(--primary-hover); }

.btn-stop {
  background: #fff;
  color: var(--danger-color);
  border: 1px solid var(--danger-color);
}

.btn-stop:hover { background: #fef2f2; }

.spinner {
  width: 16px;
  height: 16px;
  border: 2px solid rgba(255,255,255,0.3);
  border-radius: 50%;
  border-top-color: #fff;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}
</style>
