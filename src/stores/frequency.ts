import { defineStore } from 'pinia';
import { handleError } from '@tb-dev/vue';
import type { Option } from '@tb-dev/utils';
import { type DeepReadonly, ref } from 'vue';
import { commands, type Frequency } from '@/api/bindings';

export const useFrequencyStore = defineStore('frequency', () => {
  const folder = ref<Option<string>>(null);
  const search = ref<Option<string>>(null);
  const sorting = ref<Sorting>({ ascending: false });
  const selected = ref<Option<DeepReadonly<Frequency>>>(null);

  async function pickFolder() {
    try {
      folder.value = await commands.pickFolder();
    } catch (err) {
      handleError(err);
    }
  }

  return {
    folder,
    search,
    sorting,
    selected,
    pickFolder,
  };
});
