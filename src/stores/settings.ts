import { ref } from 'vue';
import { defineStore } from 'pinia';
import { localRef } from '@tb-dev/vue';
import type { Option } from '@tb-dev/utils';

export const BASE_URL_KEY = 'kanji:base-url';

export const useSettingsStore = defineStore('settings', () => {
  const baseUrl = localRef<Option<string>>(BASE_URL_KEY, null, {
    initOnMounted: false,
    listenToStorageChanges: true,
    writeDefaults: true,
  });

  const clipboard = ref(true);
  const hideOnClose = ref(false);

  const snippetLimit = ref(1000);
  const snippetMinLen = ref(5);
  const shuffleSnippets = ref(true);

  const setFileName = ref('Kanji Set.txt');
  const setSize = ref(50);

  return {
    clipboard,
    hideOnClose,
    snippetLimit,
    snippetMinLen,
    shuffleSnippets,
    setFileName,
    setSize,
    baseUrl,
  };
}, {
  tauri: {
    filterKeys: ['baseUrl'],
    filterKeysStrategy: 'omit',
  },
});
