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

  font-family: "Inter", "Segoe UI", "PingFang SC", "Hiragino Sans GB", "Microsoft YaHei", "微软雅黑", system-ui, -apple-system, sans-serif;

  font-size: 16px;

  line-height: 1.5;

  font-weight: 400;



  /* 配色方案 */

  --primary-color: #2563eb;

  --primary-hover: #1d4ed8;

  --bg-color: #f8fafc;

  --card-bg: #ffffff;

  --text-main: #1e293b;

  --text-muted: #64748b;

  --border-color: #e2e8f0;

  --success-color: #10b981;

  --danger-color: #ef4444;

  --warning-color: #f59e0b;

}



body {

  margin: 0;

  background-color: var(--bg-color);

  color: var(--text-main);

  -webkit-font-smoothing: antialiased;

}



.admin-container {

  max-width: 1100px;

  margin: 0 auto;

  padding: 40px 20px;

}



.admin-header {

  margin-bottom: 32px;

  text-align: left;

  border-left: 4px solid var(--primary-color);

  padding-left: 16px;

}



.admin-header h1 {

  margin: 0;

  font-size: 1.75rem;

  font-weight: 700;

  letter-spacing: -0.025em;

}



.admin-header p {

  margin: 4px 0 0 0;

  color: var(--text-muted);

  font-size: 0.875rem;

}



.content {

  display: flex;

  flex-direction: column;

  gap: 24px;

}



/* 通用卡片样式 */

.card {

  background: var(--card-bg);

  border-radius: 12px;

  border: 1px solid var(--border-color);

  box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px -1px rgba(0, 0, 0, 0.1);

  transition: box-shadow 0.2s ease;

}



.card:hover {

  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -2px rgba(0, 0, 0, 0.1);

}

</style>
