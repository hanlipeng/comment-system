<script setup lang="ts">
import { computed } from 'vue';
import type { PublicComment } from '../api/client';

const props = withDefaults(defineProps<{
  comments: PublicComment[];
  size?: 'normal' | 'large';
  scrollThreshold?: number;
}>(), {
  size: 'normal',
  scrollThreshold: 7
});

const shouldScroll = computed(() => props.comments.length > props.scrollThreshold);

// Double the list for seamless scrolling
const displayComments = computed(() => {
  if (props.comments.length === 0) return [];
  if (!shouldScroll.value) return props.comments;
  
  // Ensure we have enough items to scroll smoothly. 
  let list = [...props.comments];
  // For large screen, we need fewer duplications if items are tall, 
  // but to be safe, keeping logic similar ensures smooth loop.
  while (list.length < 10 && list.length > 0) {
      list = [...list, ...props.comments];
  }
  return [...list, ...list];
});

// Calculate duration based on length to keep consistent speed
const duration = computed(() => {
    if (!shouldScroll.value) return '0s';
    // Large items are taller, so we need more time to scroll through them to keep readable speed
    const perItemDuration = props.size === 'large' ? 6 : 2; 
    return `${displayComments.value.length * perItemDuration}s`;
});
</script>

<template>
  <div class="w-full h-full flex flex-col">
    <h3 v-if="size === 'normal'" class="text-center text-gray-400 text-xs uppercase tracking-widest mb-3 shrink-0">
        Latest Wishes ({{ comments.length }})
    </h3>
    
    <!-- 
      Container: 
      - normal: bg-white/50 border shadow
      - large: transparent border-none (looks cleaner on large screen) 
    -->
    <div :class="[
        'relative flex-1 overflow-hidden rounded-xl',
        size === 'normal' ? 'bg-white/50 border border-gray-100 shadow-sm' : 'bg-transparent'
    ]">
        <!-- Gradient Masks (Only show if scrolling) -->
        <template v-if="shouldScroll">
            <template v-if="size === 'large'">
                <div class="absolute top-0 left-0 right-0 h-12 bg-gradient-to-b from-white via-white/80 to-transparent z-10 pointer-events-none"></div>
                <div class="absolute bottom-0 left-0 right-0 h-12 bg-gradient-to-t from-white via-white/80 to-transparent z-10 pointer-events-none"></div>
            </template>
            
            <template v-else>
                <div class="absolute top-0 left-0 right-0 h-6 bg-gradient-to-b from-white to-transparent z-10 pointer-events-none"></div>
                <div class="absolute bottom-0 left-0 right-0 h-6 bg-gradient-to-t from-white to-transparent z-10 pointer-events-none"></div>
            </template>
        </template>

        <div v-if="comments.length > 0" 
             :class="{ 'animate-scroll-y': shouldScroll }"
             :style="{ animationDuration: duration }">
            <div v-for="(comment, idx) in displayComments" :key="idx" 
                 :class="[
                    'border-b border-gray-100', // Dashed looks cheap on large screen
                    size === 'large' ? 'px-8 py-8 border-gray-200' : 'px-4 py-3 border-dashed'
                 ]">
                
                <!-- Large Mode Layout -->
                <div v-if="size === 'large'">
                    <div class="flex justify-between items-center mb-3">
                        <span class="font-bold text-indigo-700 text-3xl">{{ comment.nickname }}</span>
                        <span class="text-xl text-gray-400 font-mono">{{ new Date(comment.createdAt).toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'}) }}</span>
                    </div>
                    <p class="text-gray-800 text-4xl leading-snug font-medium break-words">{{ comment.content }}</p>
                </div>

                <!-- Normal Mode Layout -->
                <div v-else>
                    <div class="flex justify-between items-baseline mb-1">
                        <span class="font-medium text-indigo-600 text-xs">{{ comment.nickname }}</span>
                        <span class="text-[10px] text-gray-400">{{ new Date(comment.createdAt).toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'}) }}</span>
                    </div>
                    <p class="text-gray-600 text-xs leading-relaxed break-words">{{ comment.content }}</p>
                </div>

            </div>
        </div>
        
        <div v-else class="h-full flex flex-col items-center justify-center text-gray-400">
            <p :class="size === 'large' ? 'text-3xl' : 'text-xs'">等待第一条许愿...</p>
        </div>
    </div>
  </div>
</template>

<style scoped>
@keyframes scroll-y {
  0% { transform: translateY(0); }
  100% { transform: translateY(-50%); }
}

.animate-scroll-y {
  animation: scroll-y linear infinite;
}

/* Pause on hover to allow reading */
.animate-scroll-y:hover {
    animation-play-state: paused;
}
</style>