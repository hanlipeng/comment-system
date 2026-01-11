<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { useRoute } from 'vue-router';
import { api, type EventData } from '../api/client';
import EventStatus from '../components/EventStatus.vue';
import WinnerDisplay from '../components/WinnerDisplay.vue';

const route = useRoute();
const eventData = ref<EventData | null>(null);
const loading = ref(true);
let pollTimer: any = null;

const init = async () => {
  loading.value = true;
  try {
    const idFromQuery = route.query.id as string;
    if (idFromQuery) {
      eventData.value = await api.getEvent(idFromQuery);
    } else {
      eventData.value = await api.getActiveEvent();
    }
    pollTimer = setInterval(fetchUpdate, 5000);
  } catch (e) {
    console.error(e);
  } finally {
    loading.value = false;
  }
};

const fetchUpdate = async () => {
  if (!eventData.value) return;
  try {
    eventData.value = await api.getEvent(eventData.value.id);
  } catch (e) { /* ignore */ }
};

onMounted(init);
onUnmounted(() => { if (pollTimer) clearInterval(pollTimer); });
</script>

<template>
  <div class="max-w-md mx-auto py-8 px-4">
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
          <div class="mt-8 text-center">
            <router-link to="/comment" class="text-gray-400 text-sm hover:underline">返回留言页</router-link>
          </div>
        </div>

        <div v-else :key="'waiting'" class="text-center p-12 bg-white shadow-xl rounded-2xl border border-gray-100">
          <div class="relative inline-block">
            <div class="text-6xl mb-6 animate-pulse">✨</div>
            <div class="absolute -top-1 -right-1 flex h-3 w-3">
              <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-indigo-400 opacity-75"></span>
              <span class="relative inline-flex rounded-full h-3 w-3 bg-indigo-500"></span>
            </div>
          </div>
          <h2 class="text-2xl font-bold text-gray-900 mb-2">好运正在派送中</h2>
          <p class="text-gray-500">管理员正在后台火速开奖，请勿离开此页面</p>
        </div>
      </transition>
    </div>
  </div>
</template>

<style scoped>
.scale-enter-active, .scale-leave-active { transition: all 0.6s cubic-bezier(0.34, 1.56, 0.64, 1); }
.scale-enter-from { opacity: 0; transform: scale(0.9); }
.scale-leave-to { opacity: 0; transform: scale(1.1); }
</style>