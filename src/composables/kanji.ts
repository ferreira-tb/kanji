import { storeToRefs } from 'pinia';
import * as commands from '@/commands';
import { watchImmediate } from '@vueuse/core';
import { useKanjiStore } from '@/stores/kanji';
import { isTauri } from '@tauri-apps/api/core';
import { useSettingsStore } from '@/stores/settings';
import { tryInjectOrElse, useMutex } from '@tb-dev/vue';
import { computed, effectScope, type InjectionKey, shallowRef } from 'vue';

const SYMBOL = Symbol() as InjectionKey<ReturnType<typeof create>>;

export function useKanjis() {
  return tryInjectOrElse(SYMBOL, () => {
    const scope = effectScope(/* detached */ true);
    const value = scope.run(create)!;
    return {
      kanjis: value.kanjis,
      raw: value.raw,
      currentIndex: value.currentIndex,
      loading: value.loading,
      load: value.load,
      next: value.next,
      previous: value.previous,
    };
  });
}

function create() {
  const store = useKanjiStore();
  const { sorting, search, currentKanji } = storeToRefs(store);

  const settings = useSettingsStore();
  const { baseUrl } = storeToRefs(settings);

  const { locked, lock } = useMutex();
  const raw = shallowRef<KanjiStats[]>([]);

  const kanjis = computed<KanjiStats[]>(() => {
    let result = raw.value;
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

    return result;
  });

  const currentIndex = computed(() => {
    return kanjis.value.findIndex((kanji) => {
      return kanji.character === currentKanji.value?.character;
    });
  });

  watchImmediate(kanjis, (entries) => {
    const char = currentKanji.value?.character;
    if (char && entries.every((kanji) => kanji.character !== char)) {
      currentKanji.value = null;
    }

    currentKanji.value ??= entries.at(0);
  });

  async function load() {
    await lock(async () => {
      if (isTauri() || baseUrl.value) {
        raw.value = await commands.searchKanji();
      }
      else {
        raw.value = [];
      }

      if (currentKanji.value) {
        const char = currentKanji.value.character;
        const kanji = raw.value.find((it) => it.character === char);
        if (kanji) currentKanji.value = kanji;
      }
    });
  }

  function go(index: number) {
    const size = kanjis.value.length;
    index = ((index % size) + size) % size;
    currentKanji.value = kanjis.value.at(index);
  }

  function next() {
    go(currentIndex.value + 1);
  }

  function previous() {
    go(currentIndex.value - 1);
  }

  return {
    kanjis,
    raw,
    currentIndex,
    loading: locked,
    load,
    next,
    previous,
  };
}
