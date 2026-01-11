<script setup lang="ts">
import { RouterView } from 'vue-router';
import { api } from './api/client';

// 开发工具 (仅在 npm run dev 时显示)
const showDevTools = import.meta.env.DEV;

const devDraw = async () => { await api._dev_triggerDraw(); };
const devReset = async () => { await api._dev_reset(); };
</script>

<template>
  <div class="min-h-screen bg-gray-50 text-gray-900 font-sans">
    <!-- 路由出口 -->
    <RouterView />

    <!-- 开发辅助工具 (右下角) -->
    <div class="fixed bottom-4 right-4 flex flex-col gap-2 opacity-80 z-50">
      <div class="bg-white/80 backdrop-blur p-2 rounded shadow text-xs mb-2">
        <p>Links:</p>
        <router-link to="/comment" class="text-blue-600 hover:underline block">/comment</router-link>
        <router-link to="/result" class="text-blue-600 hover:underline block">/result</router-link>
      </div>
      <button @click="devDraw" class="bg-purple-600 text-white px-3 py-1 rounded text-xs shadow hover:bg-purple-700">Dev: 开奖</button>
      <button @click="devReset" class="bg-gray-600 text-white px-3 py-1 rounded text-xs shadow hover:bg-gray-700">Dev: 重置</button>
    </div>
  </div>
</template>
