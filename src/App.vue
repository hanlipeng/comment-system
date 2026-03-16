<script setup lang="ts">
import ServerControl from "./components/ServerControl.vue";
import EventManager from "./components/EventManager.vue";
</script>

<template>
  <main class="admin-container">
    <header class="admin-header">
      <h1>留言活动管理后台</h1>
    </header>

    <section class="content">
      <ServerControl />
      <EventManager />
    </section>
  </main>
</template>

<style>

:root {
  font-family: "Geist", "Inter", "SF Pro Display", "PingFang SC", system-ui, sans-serif;
  font-size: 16px;
  line-height: 1.6;
  font-weight: 400;

  /* Refined Palette */
  --primary-color: #0f172a;
  --primary-hover: #1e293b;
  --accent-blue: #3b82f6;
  --bg-color: #f1f5f9;
  --card-bg: rgba(255, 255, 255, 0.8);
  --text-main: #0f172a;
  --text-muted: #64748b;
  --border-color: rgba(226, 232, 240, 0.8);
  --success-color: #10b981;
  --danger-color: #ef4444;
  --warning-color: #f59e0b;
  
  --glass-border: rgba(255, 255, 255, 0.4);
  --shadow-sm: 0 1px 2px 0 rgb(0 0 0 / 0.05);
  --shadow-md: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1);
  --shadow-lg: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1);
}

body {
  margin: 0;
  background-color: var(--bg-color);
  background-image: 
    radial-gradient(at 0% 0%, rgba(59, 130, 246, 0.05) 0px, transparent 50%),
    radial-gradient(at 100% 0%, rgba(16, 185, 129, 0.05) 0px, transparent 50%);
  color: var(--text-main);
  -webkit-font-smoothing: antialiased;
  min-height: 100vh;
}

/* Subtle grain texture overlay */
body::before {
  content: "";
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  opacity: 0.02;
  z-index: -1;
  pointer-events: none;
  background-image: url("data:image/svg+xml,%3Csvg viewBox='0 0 200 200' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='noiseFilter'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.65' numOctaves='3' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23noiseFilter)'/%3E%3C/svg%3E");
}

.admin-container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 60px 24px;
}

.admin-header {
  margin-bottom: 48px;
  text-align: left;
  position: relative;
}

.admin-header::after {
  content: "";
  position: absolute;
  bottom: -12px;
  left: 0;
  width: 40px;
  height: 4px;
  background: var(--accent-blue);
  border-radius: 2px;
}

.admin-header h1 {
  margin: 0;
  font-size: 2.25rem;
  font-weight: 800;
  letter-spacing: -0.04em;
  color: var(--primary-color);
}

.content {
  display: flex;
  flex-direction: column;
  gap: 32px;
  animation: slideUp 0.6s cubic-bezier(0.16, 1, 0.3, 1);
}

@keyframes slideUp {
  from { opacity: 0; transform: translateY(20px); }
  to { opacity: 1; transform: translateY(0); }
}

/* 通用卡片样式 */
.card {
  background: var(--card-bg);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
  border-radius: 16px;
  border: 1px solid var(--glass-border);
  box-shadow: var(--shadow-md);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.card:hover {
  box-shadow: var(--shadow-lg);
  transform: translateY(-2px);
  border-color: var(--accent-blue);
}

</style>
