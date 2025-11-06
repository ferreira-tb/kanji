import { defineStore } from 'pinia';
import { ref, watchEffect } from 'vue';

export const BASE_URL_KEY = 'kanji:base-url';

export const useSettingsStore = defineStore('settings', () => {
  const baseUrl = ref(localStorage.getItem(BASE_URL_KEY));

  const clipboard = ref(true);
  const hideOnClose = ref(false);

  const snippetLimit = ref(1000);
  const snippetMinLen = ref(5);
  const shuffleSnippets = ref(true);
  const ignoreSourceWeight = ref(false);

  const setFileName = ref('Kanji Set.txt');
  const setChunkSize = ref(25);

  if (__MOBILE__) {
    watchEffect(() => {
      localStorage.setItem(BASE_URL_KEY, baseUrl.value ?? '');
    });
  }

  return {
    clipboard,
    hideOnClose,
    snippetLimit,
    snippetMinLen,
    shuffleSnippets,
    ignoreSourceWeight,
    setFileName,
    setChunkSize,
    baseUrl,
  };
}, {
  tauri: {
    filterKeys: ['baseUrl'],
    filterKeysStrategy: 'omit',
  },
});
