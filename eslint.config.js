import { defineConfig } from '@tb-dev/eslint-config';

export default defineConfig({
  project: ['./tsconfig.json'],
  ignores: ['**/api/bindings.ts'],
  features: {
    vue: true,
  },
});
