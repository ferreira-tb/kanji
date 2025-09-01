import { ref } from 'vue';
import { defineStore } from 'pinia';

export const useSettingsStore = defineStore('settings', () => {
  const clipboard = ref(true);
  const hideOnClose = ref(false);

  const snippetLimit = ref(1000);
  const snippetMinLen = ref(5);
  const shuffleSnippets = ref(true);

  const setFileName = ref('Kanji Set.txt');

  return {
    clipboard,
    hideOnClose,
    snippetLimit,
    snippetMinLen,
    shuffleSnippets,
    setFileName,
  };
});
