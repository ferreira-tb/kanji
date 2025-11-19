import { defineStore } from 'pinia';
import { ref, watchEffect } from 'vue';

export const BASE_URL_KEY = 'kanji:base-url';

export const useSettingsStore = defineStore('settings', () => {
  const baseUrl = ref(localStorage.getItem(BASE_URL_KEY));
  const clipboard = ref(true);

  const editor = ref(__DEFAULT_SETTINGS__.editor);
  const hideOnClose = ref(__DEFAULT_SETTINGS__.hideOnClose);

  const snippetLimit = ref(__DEFAULT_SETTINGS__.snippetLimit);
  const snippetMinLen = ref(__DEFAULT_SETTINGS__.snippetMinLen);
  const shuffleSnippets = ref(__DEFAULT_SETTINGS__.shuffleSnippets);
  const ignoreSourceWeight = ref(__DEFAULT_SETTINGS__.ignoreSourceWeight);

  const setFileName = ref(__DEFAULT_SETTINGS__.setFileName);
  const setChunkSize = ref(__DEFAULT_SETTINGS__.setChunkSize);

  if (__MOBILE__) {
    watchEffect(() => {
      localStorage.setItem(BASE_URL_KEY, baseUrl.value ?? '');
    });
  }

  return {
    baseUrl,
    clipboard,
    editor,
    hideOnClose,
    snippetLimit,
    snippetMinLen,
    shuffleSnippets,
    ignoreSourceWeight,
    setFileName,
    setChunkSize,
  };
});
