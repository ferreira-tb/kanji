import { ref } from 'vue';
import { storeToRefs } from 'pinia';
import { useMutex } from '@tb-dev/vue';
import * as commands from '@/commands';
import { handleError } from '@/lib/error';
import { tryOnMounted } from '@vueuse/core';
import { isTauri } from '@tauri-apps/api/core';
import { useSettingsStore } from '@/stores/settings';

export function useSources() {
  const sources = ref<readonly Source[]>([]);
  const { locked, ...mutex } = useMutex();

  const settings = useSettingsStore();
  const { baseUrl } = storeToRefs(settings);

  async function loadSources() {
    try {
      await mutex.acquire();
      if (isTauri() || baseUrl.value) {
        sources.value = await commands.getSources();
      }
      else {
        sources.value = [];
      }
    }
    catch (err) {
      handleError(err);
    }
    finally {
      mutex.release();
    }
  }

  tryOnMounted(() => void loadSources());

  return {
    sources: sources as Readonly<typeof sources>,
    loading: locked,
    loadSources,
  };
}
