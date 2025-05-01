/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { storeToRefs } from 'pinia';
import { tryInjectOrElse } from '@tb-dev/vue';
import type { Fn, Option } from '@tb-dev/utils';
import { until, watchImmediate } from '@vueuse/core';
import { useSettingsStore } from '@/stores/settings';
import { useFrequencyStore } from '@/stores/frequency';
import { commands, type Frequency } from '@/api/bindings';
import { watch as watchFiles } from '@tauri-apps/plugin-fs';
import {
  computed,
  type DeepReadonly,
  effectScope,
  type InjectionKey,
  readonly,
  type Ref,
  ref,
  shallowRef,
  type ShallowRef,
} from 'vue';

interface UseFrequencyReturn {
  entries: Readonly<ShallowRef<DeepReadonly<Frequency[]>>>;
  load: () => Promise<void>;
  loading: Readonly<Ref<boolean>>;
  raw: Readonly<ShallowRef<DeepReadonly<Frequency[]>>>;
}

const SYMBOL = Symbol() as InjectionKey<UseFrequencyReturn>;

export function useFrequency() {
  return tryInjectOrElse(SYMBOL, () => {
    const scope = effectScope(/* detached */ true);
    const value = scope.run(start)!;
    return {
      entries: value.entries,
      raw: value.raw,
      load: value.load,
      loading: value.loading,
    };
  });
}

function start() {
  const store = useFrequencyStore();
  const { folder, sorting, search, selected } = storeToRefs(store);

  const settings = useSettingsStore();
  const { watchFiles: shouldWatchFiles } = storeToRefs(settings);

  const loading = ref(false);
  const raw = shallowRef<Frequency[]>([]);
  const entries = computed<Frequency[]>(() => {
    let result = raw.value;
    if (folder.value) {
      if (typeof search.value === 'string' && search.value.length > 0) {
        result = result.filter(({ kanji }) => {
          return search.value?.includes(kanji.character);
        });
      }

      result.sort(({ seen: a }, { seen: b }) => {
        return sorting.value.ascending ? a - b : b - a;
      });

      for (const { sources } of result) {
        sources.sort(({ seen: a }, { seen: b }) => {
          return sorting.value.ascending ? a - b : b - a;
        });
      }
    }

    return result;
  });

  let unwatchFiles: Option<Fn> = null;
  watchImmediate([folder, shouldWatchFiles], async () => {
    unwatchFiles?.();
    unwatchFiles = null;
    await load();

    if (shouldWatchFiles.value && folder.value) {
      const cb = () => load().err();
      // eslint-disable-next-line require-atomic-updates
      unwatchFiles = await watchFiles(folder.value, cb, {
        delayMs: 5000,
        recursive: true,
      });
    }
  });

  watchImmediate(entries, () => {
    const char = selected.value?.kanji.character;
    if (char && entries.value.every(({ kanji }) => kanji.character !== char)) {
      selected.value = null;
    }

    selected.value ??= entries.value.at(0);
  });

  async function load() {
    await until(loading).not.toBeTruthy();
    loading.value = true;

    try {
      if (folder.value) {
        raw.value = await commands.searchKanji(folder.value);
      } else if (raw.value.length > 0) {
        raw.value = [];
      }
    } finally {
      loading.value = false;
    }
  }

  return {
    entries,
    raw,
    load,
    loading: readonly(loading),
  };
}
