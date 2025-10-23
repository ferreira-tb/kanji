import '@/assets/base.css';
import '@/assets/main.css';
import '@tb-dev/vue-components/style';
import '@/lib/prototype';
import App from '@/App.vue';
import { createApp } from 'vue';
import { router } from '@/router';
import { createPinia } from 'pinia';
import { handleError } from '@/lib/error';
import { checkForUpdates } from '@/lib/updater';
import { TauriPluginPinia } from '@tauri-store/pinia';
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
  if (__DESKTOP__) {
    try {
      await checkForUpdates();
    }
    catch (err) {
      handleError(err);
    }
    finally {
      await router.push({ name: 'home' satisfies Route });
    }
  }

  app.mount('#app');
}

void init();
