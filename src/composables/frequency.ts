/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { storeToRefs } from 'pinia';
import { watchImmediate } from '@vueuse/core';
import { tryInjectOrElse } from '@tb-dev/vue';
import type { Fn, Option } from '@tb-dev/utils';
import { useSettingsStore } from '@/stores/settings';
import { commands, type Frequency } from '@/api/bindings';
import { watch as watchFiles } from '@tauri-apps/plugin-fs';
import {
  computed,
  type DeepReadonly,
  effectScope,
  type InjectionKey,
  shallowRef,
  type ShallowRef,
} from 'vue';

interface UseFrequencyReturn {
  entries: Readonly<ShallowRef<DeepReadonly<Frequency[]>>>;
  load: () => Promise<void>;
  raw: Readonly<ShallowRef<DeepReadonly<Frequency[]>>>;
}

const SYMBOL = Symbol() as InjectionKey<UseFrequencyReturn>;

export function useFrequency() {
  return tryInjectOrElse(SYMBOL, () => {
    const scope = effectScope(/* detached */ true);
    const value = scope.run(() => {
      const settings = useSettingsStore();
      const { folder, sorting, search, selected } = storeToRefs(settings);

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

      let unwatchFs: Option<Fn> = null;

      watchImmediate(folder, async (_folder) => {
        await load();
        if (_folder) {
          unwatchFs = await watchFiles(_folder, onFileEvent, {
            delayMs: 5000,
            recursive: true,
          });
        } else {
          unwatchFs?.();
          unwatchFs = null;
        }
      });

      watchImmediate(entries, (_entries) => {
        const char = selected.value?.kanji.character;
        if (char && _entries.every(({ kanji }) => kanji.character !== char)) {
          selected.value = null;
        }

        selected.value ??= _entries.at(0);
      });

      async function load() {
        if (folder.value) {
          raw.value = await commands.searchKanji(folder.value);
        } else {
          raw.value = [];
        }
      }

      function onFileEvent() {
        load().err();
      }

      return { entries, raw, load };
    });

    return {
      entries: value!.entries,
      raw: value!.raw,
      load: value!.load,
    };
  });
}
