import { defineStore } from 'pinia';
import { ref, watchEffect } from 'vue';
import type { Option } from '@tb-dev/utils';
import { isTauri } from '@tauri-apps/api/core';

export const BASE_URL_KEY = 'kanji:base-url';

export const useSettingsStore = defineStore('settings', () => {
  const baseUrl = ref<Option<string>>(null);

  const clipboard = ref(true);
  const hideOnClose = ref(false);

  const snippetLimit = ref(1000);
  const snippetMinLen = ref(5);
  const shuffleSnippets = ref(true);

  const setFileName = ref('Kanji Set.txt');
  const setSize = ref(50);

  if (!isTauri()) {
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
