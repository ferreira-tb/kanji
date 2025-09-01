import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import tailwind from '@tailwindcss/vite';
import { fileURLToPath, URL } from 'node:url';

export default defineConfig({
  plugins: [vue(), tailwind()],
  clearScreen: false,
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('src', import.meta.url)),
    },
  },
  build: {
    emptyOutDir: true,
    minify: true,
    target: 'esnext',
    sourcemap: Boolean(env.TAURI_ENV_DEBUG),
  },
  server: {
    port: 1420,
    strictPort: true,
    host: process.env.TAURI_DEV_HOST ?? false,
    watch: {
      ignored: ['**/src-tauri/**'],
    },
  },
});
