import { defineStore } from 'pinia';
import { remove } from 'es-toolkit';
import { pickFolder } from '@/commands';
import { handleError } from '@tb-dev/vue';
import type { Option } from '@tb-dev/utils';
import { type DeepReadonly, ref } from 'vue';

export const useKanjiStore = defineStore('kanji', () => {
  const folder = ref<Option<string>>(null);
  const history = ref<string[]>([]);
  const selected = ref<Option<DeepReadonly<Kanji>>>(null);

  const search = ref<Option<string>>(null);
  const sorting = ref<Sorting>({ ascending: false });

  async function set(path?: Option<string>) {
    try {
      folder.value = path ?? (await pickFolder());
      if (folder.value) {
        remove(history.value, (it) => it === folder.value);
        history.value.push(folder.value);
        while (history.value.length > 20) {
          history.value.shift();
        }
      }
    }
    catch (err) {
      handleError(err);
    }
  }

  return {
    folder,
    history,
    search,
    sorting,
    selected,
    setFolder: set,
  };
});
