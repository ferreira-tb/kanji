import { defineStore } from 'pinia';
import { ref, watchEffect } from 'vue';
import { isTauri } from '@tauri-apps/api/core';

export const BASE_URL_KEY = 'kanji:base-url';

export const useSettingsStore = defineStore('settings', () => {
  const baseUrl = ref(localStorage.getItem(BASE_URL_KEY));

  const clipboard = ref(true);
  const hideOnClose = ref(false);

  const snippetLimit = ref(1000);
  const snippetMinLen = ref(5);
  const snippetKanjiThreshold = ref(0.2);
  const shuffleSnippets = ref(true);

  const kanjiSetFileName = ref('Kanji Set.txt');
  const kanjiSetChunkSize = ref(50);

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
    snippetKanjiThreshold,
    shuffleSnippets,
    kanjiSetFileName,
    kanjiSetChunkSize,
    baseUrl,
  };
}, {
  tauri: {
    filterKeys: ['baseUrl'],
    filterKeysStrategy: 'omit',
  },
});
