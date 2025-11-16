import { createRouter, createWebHistory } from 'vue-router';

export const router = createRouter({
  history: createWebHistory(),
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
        {
          component: () => import('@/views/quiz/stats/index.vue'),
          path: 'stats',
          name: 'quiz-stats' satisfies Route,
        },
      ],
    },
    {
      component: () => import('@/views/bookmarks/index.vue'),
      name: 'bookmarks' satisfies Route,
      path: '/bookmarks',
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
    {
      component: () => import('@/views/source-groups/index.vue'),
      name: 'source-groups' satisfies Route,
      path: '/source-groups',
    },
  ],
});

export function go(to: Route) {
  void router.push({ name: to });
}
