import { createRouter, createWebHistory } from 'vue-router';
import CommentView from '../views/CommentView.vue';
import ResultView from '../views/ResultView.vue';

const routes = [
  {
    path: '/comment',
    name: 'Comment',
    component: CommentView
  },
  {
    path: '/result',
    name: 'Result',
    component: ResultView
  },
  // 重定向根目录到留言页
  {
    path: '/',
    redirect: '/comment'
  }
];

const router = createRouter({
  history: createWebHistory(),
  routes
});

export default router;
