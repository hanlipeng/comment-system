<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { adminApi } from '../api/admin';

const isRunning = ref(false);
const loading = ref(false);
const reloading = ref(false);
const restarting = ref(false);
const accessUrl = ref('');
const showCopyTip = ref(false);
const needsRestart = ref(false);

const checkStatus = async () => {
  try {
    isRunning.value = await adminApi.isServerRunning();
    if (isRunning.value) {
      await updateAccessUrl();
    }
  } catch (e) {
    console.warn("Failed to check server status", e);
  }
};

const updateAccessUrl = async () => {
  try {
    const ipPort = await adminApi.getNetworkIp();
    // Link points to /desktop as requested
    accessUrl.value = `${ipPort}/desktop`;
  } catch (e) {
    console.warn("Failed to get IP", e);
  }
};

const toggleServer = async () => {
  loading.value = true;
  try {
    if (isRunning.value) {
      await adminApi.stopServer();
      isRunning.value = false;
      accessUrl.value = '';
    } else {
      await adminApi.startServer();
      isRunning.value = true;
      // Wait a bit for server to fully bind if necessary, then get IP
      await updateAccessUrl();
    }
    needsRestart.value = false;
  } catch (e: any) {
    alert('操作失败: ' + e.toString());
  } finally {
    loading.value = false;
  }
};

const reloadSettings = async () => {
  reloading.value = true;
  try {
    const res = await adminApi.reloadSettings();
    const { changed } = res;
    
    if (changed.db_path) {
      alert("数据库路径已变更，需要重启应用后生效");
    } else if (changed.server_port || changed.wifi) {
      if (isRunning.value) {
        needsRestart.value = true;
        alert("配置已更新，需重启服务后生效");
      } else {
        alert("配置已更新，下次启动服务生效");
      }
    } else {
      alert("配置已重新加载");
    }
  } catch (e: any) {
    alert("重新加载失败: " + e.toString());
  } finally {
    reloading.value = false;
  }
};

const restartServer = async () => {
  restarting.value = true;
  try {
    await adminApi.stopServer();
    await adminApi.startServer();
    isRunning.value = true;
    await updateAccessUrl();
    needsRestart.value = false;
    alert("服务已重启");
  } catch (e: any) {
    alert("重启失败: " + e.toString());
  } finally {
    restarting.value = false;
  }
};

const copyUrl = async () => {
  try {
    await navigator.clipboard.writeText(accessUrl.value);
    showCopyTip.value = true;
    setTimeout(() => { showCopyTip.value = false; }, 2000);
  } catch (e) {
    alert('复制失败');
  }
};

onMounted(checkStatus);
</script>

<template>
  <div class="card server-control">
    <div class="header">
      <div class="title-area">
        <div class="brand-icon">
          <div class="pulse-ring" v-if="isRunning"></div>
          🌐
        </div>
        <div>
          <h3>系统状态与连接</h3>
          <p class="subtitle">管理后端服务实例与网络配置</p>
        </div>
      </div>
      <div class="status-indicator" :class="{ 'online': isRunning }">
        <span class="dot"></span>
        <span class="status-text">{{ isRunning ? '已上线' : '待命' }}</span>
      </div>
    </div>
    
    <div class="control-body">
      <div class="info-group">
        <div class="info-item">
          <span class="label">配置文件</span>
          <span class="value">settings.toml</span>
        </div>
        <div class="info-item" v-if="needsRestart">
          <span class="alert-tag">需要重启以应用变更</span>
        </div>
      </div>

      <div class="action-area">
        <button 
          @click="reloadSettings"
          class="btn-control secondary"
          :disabled="reloading || loading || restarting"
        >
          <span class="btn-icon">🔄</span>
          {{ reloading ? '同步中...' : '重载配置' }}
        </button>

        <button 
          v-if="needsRestart && isRunning"
          @click="restartServer"
          class="btn-control warning"
          :disabled="restarting || loading || reloading"
        >
          <span class="btn-icon">⚡</span>
          {{ restarting ? '重启中...' : '重启服务' }}
        </button>

        <button 
          @click="toggleServer" 
          class="btn-control main"
          :class="{ 'stop': isRunning, 'start': !isRunning }"
          :disabled="loading || restarting || reloading"
        >
          <template v-if="loading">
            <span class="spinner"></span> 
          </template>
          <template v-else>
            <span class="btn-icon">{{ isRunning ? '⏹' : '▶' }}</span>
            {{ isRunning ? '终止服务' : '启动实例' }}
          </template>
        </button>
      </div>
    </div>

    <!-- Access Link Area -->
    <transition name="fade">
      <div v-if="isRunning && accessUrl" class="access-box">
        <div class="access-icon">🚀</div>
        <div class="access-info">
          <span class="label">现场大屏 / 访问地址:</span>
          <input type="text" readonly :value="accessUrl" class="url-input"/>
        </div>
        <div class="access-actions">
          <button @click="copyUrl" class="btn-sm" :class="{ 'copied': showCopyTip }">
            {{ showCopyTip ? '已复制' : '复制' }}
          </button>
        </div>
      </div>
    </transition>
  </div>
</template>

<style scoped>
.server-control {
  padding: 32px;
  background: linear-gradient(145deg, rgba(255, 255, 255, 0.9), rgba(248, 250, 252, 0.8));
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 32px;
}

.title-area {
  display: flex;
  align-items: center;
  gap: 16px;
}

.brand-icon {
  width: 48px;
  height: 48px;
  background: var(--primary-color);
  color: white;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.5rem;
  position: relative;
  box-shadow: 0 4px 12px rgba(15, 23, 42, 0.2);
}

.pulse-ring {
  position: absolute;
  width: 100%;
  height: 100%;
  border-radius: 12px;
  border: 2px solid var(--success-color);
  animation: pulse 2s infinite;
}

@keyframes pulse {
  0% { transform: scale(1); opacity: 0.8; }
  100% { transform: scale(1.4); opacity: 0; }
}

.title-area h3 {
  margin: 0;
  font-size: 1.25rem;
  font-weight: 800;
  letter-spacing: -0.02em;
  color: var(--text-main);
}

.subtitle {
  margin: 4px 0 0 0;
  font-size: 0.85rem;
  color: var(--text-muted);
}

.status-indicator {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 0.75rem;
  font-weight: 800;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--text-muted);
  background: rgba(241, 245, 249, 0.8);
  padding: 6px 14px;
  border-radius: 30px;
  border: 1px solid var(--border-color);
}

.status-indicator.online {
  color: var(--success-color);
  background: rgba(16, 185, 129, 0.1);
  border-color: rgba(16, 185, 129, 0.2);
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
  align-items: center;
  justify-content: space-between;
}

.info-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.info-item {
  display: flex;
  align-items: center;
  gap: 12px;
}

.info-item .label {
  font-size: 0.75rem;
  font-weight: 700;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.info-item .value {
  font-family: "Geist Mono", monospace;
  font-size: 0.85rem;
  background: #f1f5f9;
  padding: 2px 8px;
  border-radius: 4px;
  color: var(--primary-color);
  font-weight: 600;
}

.alert-tag {
  font-size: 0.75rem;
  font-weight: 700;
  color: #92400e;
  background: #fef3c7;
  padding: 4px 10px;
  border-radius: 6px;
  border: 1px solid #fde68a;
}

.action-area {
  display: flex;
  gap: 12px;
}

.btn-control {
  height: 44px;
  padding: 0 20px;
  border: 1px solid transparent;
  border-radius: 10px;
  cursor: pointer;
  font-weight: 700;
  font-size: 0.9rem;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  display: flex;
  align-items: center;
  gap: 8px;
  white-space: nowrap;
}

.btn-icon { font-size: 1rem; }

.btn-control.main.start {
  background: var(--primary-color);
  color: white;
  box-shadow: 0 4px 12px rgba(15, 23, 42, 0.2);
}

.btn-control.main.start:hover {
  background: var(--primary-hover);
  transform: translateY(-2px);
  box-shadow: 0 6px 16px rgba(15, 23, 42, 0.3);
}

.btn-control.main.stop {
  background: white;
  color: var(--danger-color);
  border-color: var(--danger-color);
}

.btn-control.main.stop:hover {
  background: #fef2f2;
}

.btn-control.secondary {
  background: white;
  color: var(--text-main);
  border-color: var(--border-color);
}

.btn-control.secondary:hover {
  background: #f8fafc;
  border-color: var(--text-muted);
}

.btn-control.warning {
  background: #fffbeb;
  color: #92400e;
  border-color: #fde68a;
}

.btn-control.warning:hover {
  background: #fef3c7;
  transform: translateY(-2px);
}

.btn-control:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  transform: none !important;
  box-shadow: none !important;
}

.spinner {
  width: 16px;
  height: 16px;
  border: 2px solid rgba(0,0,0,0.1);
  border-radius: 50%;
  border-top-color: currentColor;
  animation: spin 1s linear infinite;
}

/* Access Box Styles */
.access-box {
  margin-top: 24px;
  background: #0f172a;
  border-radius: 12px;
  padding: 16px 20px;
  display: flex;
  align-items: center;
  gap: 16px;
  box-shadow: inset 0 2px 4px rgba(0,0,0,0.2);
}

.access-icon { font-size: 1.5rem; filter: drop-shadow(0 0 8px rgba(59, 130, 246, 0.5)); }

.access-info { flex: 1; display: flex; flex-direction: column; }
.access-info .label { 
  font-size: 0.7rem; 
  color: #94a3b8; 
  font-weight: 800; 
  text-transform: uppercase; 
  letter-spacing: 0.1em;
  margin-bottom: 4px; 
}
.access-info .url-input { 
  background: transparent;
  border: none;
  color: #3b82f6; 
  font-family: "Geist Mono", monospace; 
  font-size: 1rem; 
  font-weight: 600; 
  width: 100%;
  outline: none;
  cursor: text;
}

.btn-sm {
  height: 34px;
  padding: 0 14px;
  font-size: 0.75rem;
  font-weight: 800;
  background: rgba(255,255,255,0.05);
  border: 1px solid rgba(255,255,255,0.1);
  color: white;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-sm:hover { background: rgba(255,255,255,0.1); border-color: rgba(255,255,255,0.2); }
.btn-sm.copied { background: var(--success-color); border-color: var(--success-color); }

.fade-enter-active, .fade-leave-active { transition: all 0.4s cubic-bezier(0.16, 1, 0.3, 1); }
.fade-enter-from, .fade-leave-to { opacity: 0; transform: translateY(10px); }

@keyframes spin {
  to { transform: rotate(360deg); }
}
</style>
