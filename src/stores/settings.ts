import { ref } from 'vue';
import { defineStore } from 'pinia';

export const useSettingsStore = defineStore('settings', () => {
  const hideOnClose = ref(false);
  const watchFiles = ref(true);

  return {
    hideOnClose,
    watchFiles,
  };
});
