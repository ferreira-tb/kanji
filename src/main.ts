import '@/assets/style/base.css';
import '@/assets/style/vars.css';
import '@/assets/style/main.css';
import '@tb-dev/vue-components/style';
import '@/lib/prototype';
import App from '@/App.vue';
import { createApp } from 'vue';
import { router } from '@/router';
import { createPinia } from 'pinia';
import { handleError } from '@/lib/error';
import { checkForUpdates } from '@/lib/updater';
import { TauriPluginPinia } from '@tauri-store/pinia';
import { attachConsole } from '@tauri-apps/plugin-log';
import { setCurrentApp, setErrorHandler } from '@tb-dev/vue';

const app = createApp(App);
const pinia = createPinia();

if (__DESKTOP__) {
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

async function init() {
  try {
    await checkForUpdates();
    if (__DEBUG_ASSERTIONS__) {
      await attachConsole();
    }
  }
  catch (err) {
    handleError(err);
  }

  app.mount('#app');
}

void init();
