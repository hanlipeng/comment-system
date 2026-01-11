<script setup lang="ts">
import type { Winner } from '../api/client';

defineProps<{
  winner: Winner;
}>();
</script>

<template>
  <div class="bg-gradient-to-r from-yellow-100 via-yellow-200 to-yellow-100 shadow-lg rounded-lg p-8 text-center border border-yellow-300">
    <div class="text-4xl mb-4">🏆</div>
    <h2 class="text-xl font-bold text-yellow-800 mb-2">幸运大奖已诞生！</h2>
    
    <div class="py-4">
      <p class="text-lg font-semibold text-gray-900">{{ winner.nickname }}</p>
      <p class="text-sm text-gray-600 mb-2">{{ winner.phone_masked }}</p>
      <div class="bg-white/60 rounded p-4 inline-block mt-2">
        <p class="text-gray-800 italic">"{{ winner.content }}"</p>
      </div>
    </div>

    <p class="text-sm text-yellow-700 mt-4">工作人员稍后将通过手机号联系您领奖</p>
  </div>
</template>
