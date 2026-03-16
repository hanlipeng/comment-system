<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { api, type EventData, type PublicComment, ApiError } from '../api/client';
import EventStatus from '../components/EventStatus.vue';
import WinnerDisplay from '../components/WinnerDisplay.vue';
import ScrollingCommentList from '../components/ScrollingCommentList.vue';

const route = useRoute();
const router = useRouter();
const eventData = ref<EventData | null>(null);
const comments = ref<PublicComment[]>([]);
const loading = ref(true);
const showToast = ref(false);
const hasSubmitted = ref(true); // Default to true to hide button initially
let pollTimer: any = null;
let currentEventId: string | null = null;

const checkSubmission = (id: string) => {
    const key = `comment_system_event_${id}_submitted`;
    hasSubmitted.value = localStorage.getItem(key) === 'true';
};

const fetchComments = async (id: string) => {
    try {
        const list = await api.getComments(id);
        comments.value = list;
    } catch (e) {
        console.error("Failed to fetch comments", e);
    }
};

const init = async () => {
  loading.value = true;
  
  // 1. Determine Event ID
  currentEventId = (route.params.id || route.query.id) as string;
  
  // Fallback: If no ID, try to get active event
  if (!currentEventId) {
      try {
          const active = await api.getActiveEvent();
          if (active) currentEventId = active.id;
      } catch (e) {}
  }

  if (currentEventId) {
      checkSubmission(currentEventId);
      
      // Check for toast trigger
      if (route.query.new_submission === '1') {
          showToast.value = true;
          // Clear query param to prevent showing toast on refresh
          router.replace({ query: {} });
          setTimeout(() => { showToast.value = false; }, 4000);
      }

      try {
        const [data, _] = await Promise.all([
            api.getEvent(currentEventId),
            fetchComments(currentEventId)
        ]);
        eventData.value = data;
      } catch (e) {
        console.error(e);
      }
      
      startPolling();
  }
  
  loading.value = false;
};

const startPolling = () => {
  if (pollTimer) clearInterval(pollTimer);
  pollTimer = setInterval(fetchUpdate, 5000);
};

const fetchUpdate = async () => {
  if (!currentEventId) return;
  try {
    const [newEvent, _] = await Promise.all([
        api.getEvent(currentEventId),
        fetchComments(currentEventId)
    ]);
    eventData.value = newEvent;
  } catch (e: any) { 
      if (e instanceof ApiError && e.status === 404) {
          eventData.value = null; // Reset to show 'No Active Event' state
          if (pollTimer) clearInterval(pollTimer);
      }
  }
};

onMounted(init);
onUnmounted(() => { if (pollTimer) clearInterval(pollTimer); });
</script>

<template>
  <div class="max-w-md mx-auto py-8 px-4">
    <!-- Success Toast -->
    <transition name="fade">
        <div v-if="showToast" class="fixed top-6 left-1/2 transform -translate-x-1/2 z-50 bg-green-500 text-white px-6 py-3 rounded-full shadow-lg flex items-center space-x-2">
            <span class="text-xl">✅</span>
            <span class="font-medium">留言成功，等待开奖！</span>
        </div>
    </transition>

    <div v-if="loading" class="flex flex-col items-center justify-center mt-20">
      <div class="animate-bounce text-4xl">🎲</div>
      <p class="text-gray-400 mt-4">获取结果中...</p>
    </div>

    <div v-else-if="eventData">
      <EventStatus 
        :title="eventData.title" 
        :status="eventData.status" 
      />

      <transition name="scale" mode="out-in">
        <div v-if="eventData.winner" :key="'winner'">
          <WinnerDisplay :winner="eventData.winner" />
          
          <div class="mt-8 text-center" v-if="!hasSubmitted">
            <router-link :to="`/events/${eventData.id}`" class="text-indigo-600 hover:underline">我也要参与下期活动 &rarr;</router-link>
          </div>
          
          <!-- 即使开奖了，也展示留言墙作为氛围回顾 -->
          <div class="mt-8 h-64">
              <ScrollingCommentList :comments="comments" />
          </div>
        </div>

        <div v-else :key="'waiting'" class="flex flex-col gap-6">
            <div class="text-center p-8 bg-white shadow-xl rounded-2xl border border-gray-100 relative overflow-hidden">
              <div class="relative inline-block">
                <div class="text-6xl mb-6 animate-pulse">✨</div>
                <div class="absolute -top-1 -right-1 flex h-3 w-3">
                  <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-indigo-400 opacity-75"></span>
                  <span class="relative inline-flex rounded-full h-3 w-3 bg-indigo-500"></span>
                </div>
              </div>
              <h2 class="text-2xl font-bold text-gray-900 mb-2">好运正在派送中</h2>
              <p class="text-gray-500 text-sm mb-6">管理员正在后台火速开奖，请勿离开此页面</p>
              
              <div v-if="!hasSubmitted" class="border-t border-gray-100 pt-6">
                  <p class="text-xs text-gray-400 mb-3">还没有留言？</p>
                  <router-link :to="`/events/${eventData.id}`" class="inline-block bg-white text-indigo-600 border border-indigo-200 px-6 py-2 rounded-full text-sm font-semibold hover:bg-indigo-50 transition shadow-sm">
                    我也要留言
                  </router-link>
              </div>
            </div>
            <div class="h-64">
                <ScrollingCommentList :comments="comments" />
            </div>
        </div>
      </transition>
    </div>

    <div v-else class="text-center mt-20">
      <div class="text-6xl mb-4">📭</div>
      <h2 class="text-xl font-bold text-gray-800">暂无活跃活动</h2>
      <p class="text-gray-500 mt-2">管理员开启活动后即可查看结果</p>
    </div>
  </div>
</template>

<style scoped>
.scale-enter-active, .scale-leave-active { transition: all 0.6s cubic-bezier(0.34, 1.56, 0.64, 1); }
.scale-enter-from { opacity: 0; transform: scale(0.9); }
.scale-leave-to { opacity: 0; transform: scale(1.1); }

.fade-enter-active, .fade-leave-active { transition: opacity 0.5s ease, transform 0.5s ease; }
.fade-enter-from, .fade-leave-to { opacity: 0; transform: translate(-50%, -20px); }
</style>