import { isTauri } from '@tauri-apps/api/core';
import { createMemoryHistory, createRouter, createWebHistory } from 'vue-router';

export const router = createRouter({
  history: isTauri() ? createMemoryHistory() : createWebHistory(),
  routes: [
    {
      component: () => import('@/views/home/index.vue'),
      name: 'home' satisfies Route,
      path: '/',
    },
    {
      component: () => import('@/views/quiz/index.vue'),
      path: '/quiz',
      children: [
        {
          component: () => import('@/views/quiz/root/index.vue'),
          path: '',
          name: 'quiz' satisfies Route,
        },
        {
          component: () => import('@/views/quiz/history/index.vue'),
          path: 'history',
          name: 'quiz-history' satisfies Route,
        },
      ],
    },
    {
      component: () => import('@/views/settings/index.vue'),
      name: 'settings' satisfies Route,
      path: '/settings',
    },
    {
      component: () => import('@/views/snippets/index.vue'),
      name: 'snippets' satisfies Route,
      path: '/snippets',
    },
    {
      component: () => import('@/views/sources/index.vue'),
      name: 'sources' satisfies Route,
      path: '/sources',
    },
  ],
});

export function go(to: Route) {
  void router.push({ name: to });
}
