import { createRouter, createWebHistory } from 'vue-router';
import CommentView from '../views/CommentView.vue';
import ResultView from '../views/ResultView.vue';
import DesktopResultView from '../views/DesktopResultView.vue';

const routes = [
  {
    path: '/events/:id',
    name: 'Comment',
    component: CommentView
  },
  {
    path: '/events/:id/result',
    name: 'Result',
    component: ResultView
  },
  // Backward compatibility
  {
    path: '/comment/:id?',
    redirect: (to: any) => `/events/${to.params.id || '1'}`
  },
  {
    path: '/desktop',
    name: 'DesktopResult',
    component: DesktopResultView
  },
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