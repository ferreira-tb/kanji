import { createMemoryHistory, createRouter } from 'vue-router';

export const router = createRouter({
  history: createMemoryHistory(),
  routes: [
    {
      component: () => import('@/views/home/index.vue'),
      name: 'home' satisfies Route,
      path: '/',
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
