import { defineStore } from 'pinia';
import { pickFolder } from '@/commands';
import { handleError } from '@tb-dev/vue';
import type { Option } from '@tb-dev/utils';
import { type DeepReadonly, ref } from 'vue';

export const useKanjiStore = defineStore('kanji', () => {
  const folder = ref<Option<string>>(null);
  const search = ref<Option<string>>(null);
  const sorting = ref<Sorting>({ ascending: false });
  const selected = ref<Option<DeepReadonly<Kanji>>>(null);

  async function pick() {
    try {
      folder.value = await pickFolder();
    } catch (err) {
      handleError(err);
    }
  }

  return {
    folder,
    search,
    sorting,
    selected,
    pickFolder: pick,
  };
});
