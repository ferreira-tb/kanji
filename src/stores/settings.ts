import { ref } from 'vue';
import { defineStore } from 'pinia';

export const useSettingsStore = defineStore('settings', () => {
  const copyKanji = ref(true);
  const hideOnClose = ref(false);
  const watchFiles = ref(true);

  return {
    copyKanji,
    hideOnClose,
    watchFiles,
  };
});
