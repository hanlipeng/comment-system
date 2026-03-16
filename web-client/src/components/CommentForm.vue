<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { api } from '../api/client';

const props = defineProps<{
  eventId: string;
}>();

const emit = defineEmits<{
  (e: 'success'): void;
}>();

const nickname = ref('');
const content = ref('');
const phone = ref('');
const isSubmitting = ref(false);
const errorMsg = ref('');
const hasSubmitted = ref(false);

const STORAGE_KEY = `comment_system_event_${props.eventId}_submitted`;

const checkStatus = () => {
  if (localStorage.getItem(STORAGE_KEY) === 'true') {
    hasSubmitted.value = true;
  }
};

const setStatus = () => {
  localStorage.setItem(STORAGE_KEY, 'true');
  hasSubmitted.value = true;
};

onMounted(() => {
  checkStatus();
});

const handleSubmit = async () => {
  if (hasSubmitted.value) return;
  if (!nickname.value || !content.value || !phone.value) return;
  
  errorMsg.value = '';

  // 前端预校验手机号
  if (!/^1[3-9]\d{9}$/.test(phone.value)) {
    errorMsg.value = '请输入正确的手机号';
    return;
  }

  isSubmitting.value = true;

  try {
    await api.postComment(props.eventId, {
      nickname: nickname.value,
      content: content.value,
      phone: phone.value
    });
    
    setStatus(); // Mark as submitted
    
    // Emit success for parent to handle redirection
    emit('success');
    
  } catch (e: any) {
    errorMsg.value = e.message || '提交失败，请检查网络';
    isSubmitting.value = false;
  }
};
</script>

<template>
  <div class="bg-white shadow-2xl rounded-2xl p-8 border border-gray-50">
    <div class="flex items-center space-x-2 mb-6">
      <div class="w-1.5 h-6 bg-indigo-600 rounded-full"></div>
      <h2 class="text-xl font-bold text-gray-900">参与互动</h2>
    </div>

    <div v-if="hasSubmitted" class="text-center p-6 bg-gray-50 rounded-xl border border-gray-100">
       <span class="text-3xl mb-2 block">🎉</span>
       <h3 class="text-lg font-bold text-gray-800">您已参与过本次活动</h3>
       <p class="text-sm text-gray-500 mt-2 mb-4">请耐心等待开奖结果，祝您好运！</p>
       <router-link :to="`/events/${eventId}/result`" class="inline-block bg-indigo-600 text-white px-6 py-2 rounded-full text-sm font-semibold hover:bg-indigo-700 transition shadow-md">
          查看实时结果
       </router-link>
    </div>
    
    <form v-else @submit.prevent="handleSubmit" class="space-y-5">
      <div class="group">
        <label class="block text-xs font-bold text-gray-500 uppercase tracking-wider mb-1 px-1">您的昵称</label>
        <input 
          v-model="nickname" 
          type="text" 
          required
          class="w-full rounded-xl border-gray-200 bg-gray-50 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 text-sm p-3 transition border"
          placeholder="怎么称呼您？"
        />
      </div>

      <div>
        <label class="block text-xs font-bold text-gray-500 uppercase tracking-wider mb-1 px-1">手机号码 (仅领奖用)</label>
        <input 
          v-model="phone" 
          type="tel" 
          required
          class="w-full rounded-xl border-gray-200 bg-gray-50 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 text-sm p-3 transition border"
          placeholder="请输入11位手机号"
        />
      </div>

      <div>
        <label class="block text-xs font-bold text-gray-500 uppercase tracking-wider mb-1 px-1">留言内容</label>
        <textarea 
          v-model="content" 
          required
          rows="4"
          maxlength="200"
          class="w-full rounded-xl border-gray-200 bg-gray-50 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 text-sm p-3 transition border resize-none"
          placeholder="写下您的精彩留言..."
        ></textarea>
        <div class="text-right text-[10px] text-gray-400 mt-1">{{ content.length }}/200</div>
      </div>

      <transition name="fade">
        <div v-if="errorMsg" class="flex items-center space-x-2 text-red-600 bg-red-50 p-3 rounded-lg text-sm border border-red-100">
          <span class="text-lg">❌</span>
          <span>{{ errorMsg }}</span>
        </div>
      </transition>

      <button 
        type="submit" 
        :disabled="isSubmitting"
        class="w-full flex justify-center py-3.5 px-4 rounded-xl shadow-lg text-sm font-bold text-white bg-indigo-600 hover:bg-indigo-700 hover:shadow-indigo-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 transition-all duration-300 disabled:bg-gray-300 disabled:shadow-none transform active:scale-[0.98]"
      >
        <template v-if="isSubmitting">
          <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
          正在发送...
        </template>
        <span v-else>立即发送留言</span>
      </button>
    </form>
    
    <p class="mt-6 text-center text-[10px] text-gray-400">
      * 您的隐私对我们至关重要，手机号仅用于中奖通知
    </p>
  </div>
</template>

<style scoped>
.fade-enter-active, .fade-leave-active { transition: opacity 0.3s; }
.fade-enter-from, .fade-leave-to { opacity: 0; }
</style>