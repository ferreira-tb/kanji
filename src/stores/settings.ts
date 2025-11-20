import { ref } from 'vue';
import { defineStore } from 'pinia';
import { handleError } from '@/lib/error';
import { useLocalStorage } from '@vueuse/core';
import { BASE_URL_KEY } from '@/lib/local-storage';

export const useSettingsStore = defineStore('settings', () => {
  const baseUrl = useLocalStorage(BASE_URL_KEY, '192.168.1.65:63500', {
    initOnMounted: false,
    listenToStorageChanges: false,
    writeDefaults: true,
    onError: handleError,
  });

  const clipboard = ref(__DEFAULT_SETTINGS__.clipboard);
  const editor = ref(__DEFAULT_SETTINGS__.editor);
  const hideOnClose = ref(__DEFAULT_SETTINGS__.hideOnClose);

  const snippetLimit = ref(__DEFAULT_SETTINGS__.snippetLimit);
  const snippetMinLen = ref(__DEFAULT_SETTINGS__.snippetMinLen);
  const shuffleSnippets = ref(__DEFAULT_SETTINGS__.shuffleSnippets);
  const ignoreSourceWeight = ref(__DEFAULT_SETTINGS__.ignoreSourceWeight);

  const setFileName = ref(__DEFAULT_SETTINGS__.setFileName);
  const setChunkSize = ref(__DEFAULT_SETTINGS__.setChunkSize);

  return {
    clipboard,
    editor,
    hideOnClose,
    snippetLimit,
    snippetMinLen,
    shuffleSnippets,
    ignoreSourceWeight,
    setFileName,
    setChunkSize,

    // Mobile
    baseUrl,
  };
}, {
  tauri: {
    filterKeys: ['baseUrl'],
    filterKeysStrategy: 'omit',
  },
});
