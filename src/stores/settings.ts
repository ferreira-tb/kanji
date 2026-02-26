import { ref } from 'vue';
import { defineStore } from 'pinia';
import { handleError } from '@/lib/error';
import { useLocalStorage } from '@vueuse/core';
import { BASE_URL_KEY } from '@/lib/local-storage';

export const useSettingsStore = defineStore('settings', () => {
  const baseUrl = useLocalStorage(BASE_URL_KEY, '192.168.1.69:63500', {
    initOnMounted: false,
    listenToStorageChanges: false,
    writeDefaults: true,
    onError: handleError,
  });

  const clipboard = ref(false);
  const editor = ref<Editor>('code');
  const forbiddenWords = ref('');
  const hideOnClose = ref(false);
  const ignoreSourceWeight = ref(false);
  const setChunkSize = ref(25);
  const setFileName = ref('Kanji Set.txt');
  const shuffleSnippets = ref(true);
  const snippetLimit = ref(1000);
  const snippetMinLen = ref(5);

  return {
    // Desktop
    clipboard,
    editor,
    forbiddenWords,
    hideOnClose,
    ignoreSourceWeight,
    setChunkSize,
    setFileName,
    shuffleSnippets,
    snippetLimit,
    snippetMinLen,

    // Mobile
    baseUrl,
  };
}, {
  tauri: {
    filterKeys: ['baseUrl'],
    filterKeysStrategy: 'omit',
  },
});
