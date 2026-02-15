import '@tb-dev/vue-sonner/style.css';
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

pinia.use(
  TauriPluginPinia({
    autoStart: true,
    saveOnChange: true,
    saveStrategy: 'debounce',
    saveInterval: 500,
  }),
);

setCurrentApp(app);
setErrorHandler(handleError, app);

app
  .use(pinia)
  .use(router);

try {
  await checkForUpdates();
  if (__DEBUG_ASSERTIONS__ && __DESKTOP__) {
    await attachConsole();
  }
}
catch (err) {
  handleError(err);
}

app.mount('#app');
