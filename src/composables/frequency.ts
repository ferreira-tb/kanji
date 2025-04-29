/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { storeToRefs } from 'pinia';
import { watchImmediate } from '@vueuse/core';
import { useSettingsStore } from '@/stores/settings';
import { commands, type Frequency } from '@/api/bindings';
import { handleError, tryInjectOrElse } from '@tb-dev/vue';
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

      watchImmediate(folder, load);

      watchImmediate(entries, (_entries) => {
        const char = selected.value?.kanji.character;
        if (char && _entries.every(({ kanji }) => kanji.character !== char)) {
          selected.value = null;
        }

        selected.value ??= _entries.at(0);
      });

      async function load() {
        try {
          raw.value = folder.value ? await commands.searchKanji(folder.value) : [];
        } catch (err) {
          handleError(err);
          raw.value = [];
        }
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
