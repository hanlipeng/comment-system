<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{
  title: string;
  status: 'active' | 'closed';
}>();

const statusText = computed(() => props.status === 'active' ? '进行中' : '已结束');
const statusColor = computed(() => props.status === 'active' ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-800');
</script>

<template>
  <div class="bg-white shadow rounded-lg p-6 mb-4 text-center">
    <h1 class="text-2xl font-bold text-gray-900 mb-2">{{ title }}</h1>
    <span :class="`inline-flex items-center px-3 py-0.5 rounded-full text-sm font-medium ${statusColor}`">
      {{ statusText }}
    </span>
  </div>
</template>
