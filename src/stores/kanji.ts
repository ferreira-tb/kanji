import { defineStore } from 'pinia';
import { type DeepReadonly, ref } from 'vue';

export const useKanjiStore = defineStore('kanji', () => {
  const currentKanji = ref<Option<DeepReadonly<KanjiStats>>>(null);
  const currentSource = ref<Option<KanjiStatsSource>>(null);

  const search = ref<Option<string>>(null);
  const sorting = ref<Sorting>({ ascending: false });

  return {
    search,
    sorting,
    currentKanji,
    currentSource,
  };
});
