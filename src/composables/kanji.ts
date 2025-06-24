import { storeToRefs } from 'pinia';
import { searchKanji } from '@/commands';
import { tryInjectOrElse } from '@tb-dev/vue';
import { useKanjiStore } from '@/stores/kanji';
import { until, watchImmediate } from '@vueuse/core';
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

interface UseKanjiReturn {
  kanjis: Readonly<ShallowRef<DeepReadonly<Kanji[]>>>;
  load: () => Promise<void>;
  loading: Readonly<Ref<boolean>>;
  raw: Readonly<ShallowRef<DeepReadonly<Kanji[]>>>;
}

const SYMBOL = Symbol() as InjectionKey<UseKanjiReturn>;

export function useKanjis() {
  return tryInjectOrElse(SYMBOL, () => {
    const scope = effectScope(/* detached */ true);
    const value = scope.run(start)!;
    return {
      kanjis: value.kanjis,
      raw: value.raw,
      load: value.load,
      loading: value.loading,
    };
  });
}

function start() {
  const store = useKanjiStore();
  const { folder, sorting, search, selected } = storeToRefs(store);

  const loading = ref(false);
  const raw = shallowRef<Kanji[]>([]);
  const kanjis = computed<Kanji[]>(() => {
    let result = raw.value;
    if (folder.value) {
      if (search.value) {
        result = result.filter(({ character }) => {
          return search.value?.includes(character);
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

  watchImmediate(kanjis, () => {
    const char = selected.value?.character;
    if (char && kanjis.value.every((kanji) => kanji.character !== char)) {
      selected.value = null;
    }

    selected.value ??= kanjis.value.at(0);
  });

  async function load() {
    await until(loading).not.toBeTruthy();
    loading.value = true;

    try {
      if (folder.value) {
        raw.value = await searchKanji(folder.value);
      } else if (raw.value.length > 0) {
        raw.value = [];
      }
    } finally {
      loading.value = false;
    }
  }

  return {
    kanjis,
    raw,
    load,
    loading: readonly(loading),
  };
}
