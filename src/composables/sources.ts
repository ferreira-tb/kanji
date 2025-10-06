import { storeToRefs } from 'pinia';
import * as commands from '@/commands';
import { handleError } from '@/lib/error';
import { tryOnMounted } from '@vueuse/core';
import { isTauri } from '@tauri-apps/api/core';
import { useSettingsStore } from '@/stores/settings';
import { tryInjectOrElse, useMutex } from '@tb-dev/vue';
import { effectScope, type InjectionKey, ref } from 'vue';

const SYMBOL = Symbol() as InjectionKey<ReturnType<typeof create>>;

export function useSources() {
  return tryInjectOrElse(SYMBOL, () => {
    const scope = effectScope(/* detached */ true);
    const value = scope.run(create)!;
    return {
      sources: value.sources,
      loading: value.loading,
      loadSources: value.loadSources,
      findSource: value.findSource,
    };
  });
}

export function create() {
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

  function findSource(id: SourceId): Option<Source> {
    return sources.value.find((source) => source.id === id);
  }

  tryOnMounted(() => void loadSources());

  return {
    sources: sources as Readonly<typeof sources>,
    loading: locked,
    loadSources,
    findSource,
  };
}
