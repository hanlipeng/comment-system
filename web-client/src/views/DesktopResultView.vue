<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue';
import { useRoute } from 'vue-router';
import { api, type EventData, type PublicComment, type WifiConfig, ApiError } from '../api/client';
import ScrollingCommentList from '../components/ScrollingCommentList.vue';
import QrcodeVue from 'qrcode.vue';

const route = useRoute();
const eventData = ref<EventData | null>(null);
const comments = ref<PublicComment[]>([]);
const wifiConfig = ref<WifiConfig | null>(null);
const loading = ref(true);
let pollTimer: any = null;

const commentUrl = computed(() => {
    const baseUrl = `${window.location.protocol}//${window.location.host}/comment`;
    if (eventData.value?.id) {
        return `${baseUrl}/${eventData.value.id}`;
    }
    return baseUrl;
});

const wifiQrCodeString = computed(() => {
    if (!wifiConfig.value) return '';
    // WIFI:S:<SSID>;T:<Encryption>;P:<Password>;;
    const { ssid, encryption, password } = wifiConfig.value;
    return `WIFI:S:${ssid};T:${encryption};P:${password || ''};;`;
});

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
  try {
    // Parallel fetch for initial data
    const [wifiData, idFromQuery] = await Promise.all([
        api.getWifiConfig().catch(e => { console.warn("Wifi config load failed", e); return null; }),
        Promise.resolve(route.query.id as string)
    ]);
    
    wifiConfig.value = wifiData;

    if (idFromQuery) {
      eventData.value = await api.getEvent(idFromQuery);
    } else {
      eventData.value = await api.getActiveEvent();
    }
    
    if (eventData.value) {
        await fetchComments(eventData.value.id);
    }

    pollTimer = setInterval(fetchUpdate, 1000);
  } catch (e) {
    console.error(e);
  } finally {
    loading.value = false;
  }
};

const fetchUpdate = async () => {
  if (!eventData.value) {
      // If we don't have an event, try to find an active one periodically
      try {
          const active = await api.getActiveEvent();
          if (active) {
              eventData.value = active;
              await fetchComments(active.id);
          }
      } catch (e) {}
      return;
  }

  try {
    const [newEvent, _] = await Promise.all([
        api.getEvent(eventData.value.id),
        fetchComments(eventData.value.id)
    ]);
    eventData.value = newEvent;
  } catch (e: any) {
      if (e instanceof ApiError && e.status === 404) {
          // Event deleted, clear current data
          eventData.value = null;
          comments.value = [];
          // Try to fetch new active event immediately
          try {
             const active = await api.getActiveEvent();
             if (active) {
                 eventData.value = active;
                 await fetchComments(active.id);
             }
          } catch(ignore) {}
      }
  }
};

onMounted(init);
onUnmounted(() => { if (pollTimer) clearInterval(pollTimer); });
</script>

<template>
  <div class="h-screen w-screen bg-gray-50 flex items-center justify-center p-[5vh] overflow-hidden font-sans">
    
    <!-- Loading State -->
    <div v-if="loading" class="flex flex-col items-center justify-center">
      <div class="animate-bounce text-9xl mb-8">🎲</div>
      <p class="text-gray-400 text-4xl font-light">Loading...</p>
    </div>

    <div v-else-if="eventData" class="w-full h-full grid grid-cols-12 gap-12">
      
      <!-- Left Column: Core Info (Col 5) -->
      <div class="col-span-5 flex flex-col h-full gap-8">
        
        <!-- Header / Status -->
        <div class="bg-white rounded-3xl p-10 shadow-xl border border-gray-100 flex flex-col justify-center items-start shrink-0">
             <div class="flex items-center gap-4 mb-4">
                <span class="px-4 py-2 rounded-full text-2xl font-bold uppercase tracking-wider text-white"
                      :class="eventData.status === 'active' ? 'bg-green-500' : 'bg-red-500'">
                    {{ eventData.status === 'active' ? '进行中' : '已结束' }}
                </span>
                <span class="text-gray-400 text-2xl">#活动现场</span>
             </div>
             <h1 class="text-6xl font-black text-gray-900 leading-tight">
                 {{ eventData.title }}
             </h1>
        </div>

        <!-- Main Display Area -->
        <div class="flex-1 bg-white rounded-3xl shadow-xl border border-gray-100 p-10 flex flex-col justify-center items-center relative overflow-hidden">
             
             <!-- Winner Mode -->
             <div v-if="eventData.winner" class="w-full h-full flex flex-col justify-center items-center text-center">
                <div class="text-indigo-600 text-3xl font-bold mb-8 uppercase tracking-[0.2em]">Winner</div>
                
                <div class="animate-bounce mb-8 text-8xl">👑</div>
                
                <div class="text-7xl font-black text-gray-900 mb-6 truncate max-w-full">
                    {{ eventData.winner.nickname }}
                </div>
                <div class="text-4xl text-gray-500 font-mono bg-gray-100 px-6 py-3 rounded-xl mb-8">
                    {{ eventData.winner.phoneMasked }}
                </div>
                <div class="text-3xl text-gray-800 italic font-serif relative px-12">
                    <span class="absolute top-0 left-0 text-6xl text-gray-300">“</span>
                    {{ eventData.winner.content }}
                    <span class="absolute bottom-0 right-0 text-6xl text-gray-300">”</span>
                </div>
             </div>

             <!-- Waiting Mode -->
             <div v-else class="text-center">
                 <div class="relative inline-block mb-12">
                    <div class="text-[10rem] animate-pulse">🎁</div>
                 </div>
                 <h2 class="text-6xl font-bold text-gray-900 mb-6">等待开奖</h2>
                 <p class="text-4xl text-gray-400 font-light">
                    请关注大屏幕<br>好运即将降临
                 </p>
             </div>
        </div>

        <!-- Bottom Area: Wifi & Participation Codes -->
        <div class="flex flex-col gap-2 shrink-0">
             <div class="text-center text-gray-500 text-sm font-medium bg-white/50 py-1 rounded-full border border-gray-100">
                🚀 第一步：扫左侧码连 Wi-Fi &nbsp; → &nbsp; 第二步：扫右侧码参与活动
             </div>
             
             <div class="flex gap-6 h-40">
                <!-- Wifi Card (Optional) -->
                <div v-if="wifiConfig" class="flex-1 bg-white rounded-3xl p-4 shadow-xl border border-gray-100 flex items-center gap-4">
                    <div class="bg-blue-50 p-2 rounded-xl shrink-0">
                        <QrcodeVue :value="wifiQrCodeString" :size="90" level="Q" background="#eff6ff" foreground="#1e40af" />
                    </div>
                    <div class="flex flex-col min-w-0">
                        <h3 class="text-xl font-bold text-gray-900 mb-1">① 连 Wi-Fi</h3>
                        <p class="text-base text-gray-500 font-mono truncate" :title="wifiConfig.ssid">{{ wifiConfig.ssid }}</p>
                    </div>
                </div>

                <!-- Participation Card -->
                <div v-if="eventData.status === 'active' && !eventData.winner" class="flex-[1.2] bg-white rounded-3xl p-4 shadow-xl border border-gray-100 flex items-center gap-4">
                    <div class="bg-gray-900 p-2 rounded-xl shrink-0">
                        <QrcodeVue :value="commentUrl" :size="90" level="H" background="#ffffff" foreground="#000000" />
                    </div>
                    <div class="flex flex-col min-w-0">
                        <h3 class="text-xl font-bold text-gray-900 mb-1">② 扫码许愿</h3>
                        <p class="text-base text-gray-500">发送弹幕抽奖</p>
                    </div>
                </div>
             </div>
        </div>

      </div>

      <!-- Right Column: Comments (Col 7) -->
      <div class="col-span-7 h-full bg-white rounded-3xl shadow-xl border border-gray-100 p-8 flex flex-col overflow-hidden">
          <div class="flex items-center justify-between mb-6 px-4">
              <h2 class="text-4xl font-bold text-gray-800">实时许愿墙</h2>
              <div class="flex items-center gap-2">
                  <span class="w-3 h-3 bg-green-500 rounded-full animate-pulse"></span>
                  <span class="text-xl text-gray-500">Live Updating</span>
              </div>
          </div>
          <div class="flex-1 min-h-0 relative">
            <ScrollingCommentList :comments="comments" size="large" />
          </div>
      </div>

    </div>

    <!-- No Event State -->
    <div v-else class="flex flex-col items-center justify-center">
      <div class="text-9xl mb-12">📭</div>
      <h2 class="text-6xl font-bold text-gray-800 mb-6">暂无活跃活动</h2>
      <p class="text-4xl text-gray-400 font-light">请等待管理员开启新活动</p>
    </div>
  </div>
</template>

<style scoped>
/* No specific styles needed, using Tailwind */
</style>
