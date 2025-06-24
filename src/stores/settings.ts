import { ref } from 'vue';
import { defineStore } from 'pinia';

export const useSettingsStore = defineStore('settings', () => {
  const clipboard = ref(true);
  const hideOnClose = ref(false);

  return {
    clipboard,
    hideOnClose,
  };
});
