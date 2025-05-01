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
  ],
});

export function go(to: Route) {
  void router.push({ name: to });
}
