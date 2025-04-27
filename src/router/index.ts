import { createMemoryHistory, createRouter } from 'vue-router';

export const router = createRouter({
  history: createMemoryHistory(),
  routes: [
    {
      component: () => import('@/views/home/index.vue'),
      name: 'home' satisfies Route,
      path: '/',
    },
  ],
});

export function go(to: Route) {
  void router.push({ name: to });
}
