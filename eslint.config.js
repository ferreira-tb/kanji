import { defineConfig } from '@tb-dev/eslint-config';

export default defineConfig({
  project: ['./tsconfig.json'],
  features: {
    vue: true,
  },
  overrides: {
    typescript: {
      '@typescript-eslint/no-non-null-assertion': 'off',
    },
    perfectionist: {
      'perfectionist/sort-interfaces': 'off',
      'perfectionist/sort-object-types': 'off',
    },
    vue: {
      'vue/no-static-inline-styles': 'off',
    },
  },
});
