import { ref } from 'vue';
import { useMutex } from '@tb-dev/vue';
import { getSources } from '@/commands';
import { handleError } from '@/lib/error';
import { tryOnMounted } from '@vueuse/core';

export function useSources() {
  const sources = ref<readonly Source[]>([]);
  const { locked, ...mutex } = useMutex();

  async function loadSources() {
    try {
      await mutex.acquire();
      sources.value = await getSources();
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
