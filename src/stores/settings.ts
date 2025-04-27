import { ref } from 'vue';
import { defineStore } from 'pinia';
import { commands } from '@/api/bindings';
import { handleError } from '@tb-dev/vue';
import type { Option } from '@tb-dev/utils';

export const useSettingsStore = defineStore('settings', () => {
  const folder = ref<Option<string>>(null);
  const search = ref<Option<string>>(null);
  const sorting = ref<Sorting>({ ascending: false });

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
    pickFolder,
  };
});
