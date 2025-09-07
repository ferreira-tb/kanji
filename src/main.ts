import '@/assets/base.css';
import '@/assets/main.css';
import '@tb-dev/vue-components/style';
import '@/lib/prototype';
import App from '@/App.vue';
import { createApp } from 'vue';
import { createPinia } from 'pinia';
import { go, router } from '@/router';
import { handleError } from '@/lib/error';
import { isTauri } from '@tauri-apps/api/core';
import { checkForUpdates } from '@/lib/updater';
import { TauriPluginPinia } from '@tauri-store/pinia';
import { setCurrentApp, setErrorHandler } from '@tb-dev/vue';

const app = createApp(App);
const pinia = createPinia();

if (isTauri()) {
  pinia.use(
    TauriPluginPinia({
      autoStart: true,
      saveOnChange: true,
      saveStrategy: 'debounce',
      saveInterval: 1000,
    }),
  );
}

setCurrentApp(app);
setErrorHandler(handleError, app);

app.use(router);
app.use(pinia);

checkForUpdates()
  .then(() => go('home'))
  .then(() => app.mount('#app'))
  .err();
