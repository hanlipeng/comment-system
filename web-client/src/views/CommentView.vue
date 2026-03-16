<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { api, type EventData, ApiError } from '../api/client';
import EventStatus from '../components/EventStatus.vue';
import CommentForm from '../components/CommentForm.vue';

const route = useRoute();
const router = useRouter();
const eventData = ref<EventData | null>(null);
const loading = ref(true);
const error = ref<string | null>(null);
let pollTimer: any = null;
let currentEventId: string | null = null;

const init = async () => {
  loading.value = true;
  error.value = null;
  
  // 1. 获取 ID
  currentEventId = (route.params.id || route.query.id) as string;
  if (!currentEventId) {
      error.value = "请通过扫描现场二维码参与活动";
      loading.value = false;
      return;
  }

  // 2. 初始请求
  try {
    eventData.value = await api.getEvent(currentEventId);
  } catch (e: any) {
    handleError(e);
  } finally {
    loading.value = false;
    // 3. 无论成功失败，只要有 ID 就启动轮询（用于恢复或更新状态）
    startPolling();
  }
};

const startPolling = () => {
  if (pollTimer) clearInterval(pollTimer);
  pollTimer = setInterval(fetchUpdate, 5000);
};

const fetchUpdate = async () => {
  if (!currentEventId) return;
  
  // 如果页面不可见，暂停轮询（可选优化，暂不加，保持简单）
  
  try {
    const data = await api.getEvent(currentEventId);
    eventData.value = data;
    error.value = null; // 清除之前的错误（如网络恢复）
  } catch (e: any) {
    handleError(e);
  }
};

const handleError = (e: any) => {
    if (e instanceof ApiError && e.status === 404) {
       error.value = "抱歉，该活动已结束或被移除";
       eventData.value = null; 
       // 404 通常意味着活动真没了，停止轮询节省资源
       if (pollTimer) clearInterval(pollTimer);
    } else {
       // 网络错误等，保持轮询重试，仅在首次加载时显示错误
       console.warn("Update failed", e);
       if (!eventData.value) {
           error.value = "加载失败，正在重试...";
       }
    }
};

const handleSuccess = () => {
    if (currentEventId) {
        // 使用 replace 防止后退重复提交
        router.replace({
            path: `/events/${currentEventId}/result`,
            query: { new_submission: '1' }
        });
    }
};

onMounted(init);
onUnmounted(() => { if (pollTimer) clearInterval(pollTimer); });
</script>

<template>
  <div class="max-w-md mx-auto py-8 px-4 transition-all duration-500">
    <div v-if="loading" class="flex flex-col items-center justify-center mt-20 space-y-4">
      <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-indigo-600"></div>
      <p class="text-gray-500 font-medium">加载活动中...</p>
    </div>

    <div v-else-if="error" class="text-center mt-20">
      <div class="text-6xl mb-4">📭</div>
      <h2 class="text-xl font-bold text-gray-800">{{ error }}</h2>
      <p class="text-gray-500 mt-2">请确认链接是否正确或稍后重试</p>
    </div>

    <div v-else-if="eventData">
      <EventStatus 
        :title="eventData.title" 
        :status="eventData.status" 
      />
      
      <!-- 如果已经开奖，提示去结果页 -->
      <transition name="fade">
        <div v-if="eventData.winner" class="mb-6 p-4 bg-indigo-50 border border-indigo-100 rounded-xl text-center shadow-sm">
          <p class="text-indigo-900 font-bold mb-2">🎉 本场抽奖已圆满结束</p>
          <router-link :to="`/events/${eventData.id}/result`" class="inline-block bg-indigo-600 text-white px-6 py-2 rounded-full text-sm font-semibold hover:bg-indigo-700 transition shadow-md">
            前往查看中奖名单
          </router-link>
        </div>
      </transition>

      <div v-if="eventData.status === 'active' && !eventData.winner">
        <CommentForm :event-id="eventData.id" @success="handleSuccess" />
      </div>
      
      <div v-else-if="!eventData.winner" class="text-center p-12 bg-white shadow-xl rounded-2xl text-gray-500 border border-gray-100">
        <p class="text-lg">活动已停止收集留言</p>
        <p class="text-sm mt-2">请等待管理员开奖...</p>
        <router-link :to="`/events/${eventData.id}/result`" class="mt-4 inline-block text-indigo-600 hover:underline">
            去等待结果 &rarr;
        </router-link>
      </div>
    </div>
  </div>
</template>

<style scoped>
.fade-enter-active, .fade-leave-active { transition: opacity 0.5s ease; }
.fade-enter-from, .fade-leave-to { opacity: 0; }
</style>
