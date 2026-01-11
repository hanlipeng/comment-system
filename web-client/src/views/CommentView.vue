<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { useRoute } from 'vue-router';
import { api, type EventData } from '../api/client';
import EventStatus from '../components/EventStatus.vue';
import CommentForm from '../components/CommentForm.vue';

const route = useRoute();
const eventData = ref<EventData | null>(null);
const loading = ref(true);
const error = ref<string | null>(null);
let pollTimer: any = null;

const init = async () => {
  loading.value = true;
  error.value = null;
  try {
    const idFromQuery = route.query.id as string;
    if (idFromQuery) {
      eventData.value = await api.getEvent(idFromQuery);
    } else {
      eventData.value = await api.getActiveEvent();
    }
    
    // 开始轮询
    pollTimer = setInterval(fetchUpdate, 5000);
  } catch (e: any) {
    error.value = "未找到相关活动或活动已失效";
    console.error(e);
  } finally {
    loading.value = false;
  }
};

const fetchUpdate = async () => {
  if (!eventData.value) return;
  try {
    const data = await api.getEvent(eventData.value.id);
    eventData.value = data;
  } catch (e) {
    console.warn("Poll update failed", e);
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
      <p class="text-gray-500 mt-2">请确认链接是否正确</p>
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
          <router-link to="/result" class="inline-block bg-indigo-600 text-white px-6 py-2 rounded-full text-sm font-semibold hover:bg-indigo-700 transition shadow-md">
            前往查看中奖名单
          </router-link>
        </div>
      </transition>

      <div v-if="eventData.status === 'active' && !eventData.winner">
        <CommentForm :event-id="eventData.id" />
      </div>
      
      <div v-else-if="!eventData.winner" class="text-center p-12 bg-white shadow-xl rounded-2xl text-gray-500 border border-gray-100">
        <p class="text-lg">活动已停止收集留言</p>
        <p class="text-sm mt-2">请等待管理员开奖...</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.fade-enter-active, .fade-leave-active { transition: opacity 0.5s ease; }
.fade-enter-from, .fade-leave-to { opacity: 0; }
</style>
