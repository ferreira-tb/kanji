import { storeToRefs } from 'pinia';
import * as commands from '@/commands';
import { watchImmediate } from '@vueuse/core';
import { useKanjiStore } from '@/stores/kanji';
import { tryInjectOrElse, useMutex } from '@tb-dev/vue';
import {
  computed,
  type DeepReadonly,
  effectScope,
  type InjectionKey,
  type Ref,
  shallowRef,
  type ShallowRef,
} from 'vue';

interface UseKanjiReturn {
  kanjis: Readonly<ShallowRef<DeepReadonly<Kanji[]>>>;
  raw: Readonly<ShallowRef<DeepReadonly<Kanji[]>>>;
  currentIndex: Readonly<Ref<number>>;
  loading: Readonly<Ref<boolean>>;
  load: () => Promise<void>;
  next: () => void;
  previous: () => void;
  exportSet: () => Promise<void>;
}

const SYMBOL = Symbol() as InjectionKey<UseKanjiReturn>;

export function useKanjis() {
  return tryInjectOrElse(SYMBOL, () => {
    const scope = effectScope(/* detached */ true);
    const value = scope.run(start)!;
    return {
      kanjis: value.kanjis,
      raw: value.raw,
      currentIndex: value.currentIndex,
      loading: value.loading,
      load: value.load,
      next: value.next,
      previous: value.previous,
      exportSet: value.exportSet,
    };
  });
}

function start() {
  const store = useKanjiStore();
  const { folder, sorting, search, selected } = storeToRefs(store);

  const { locked, lock } = useMutex();
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

  const currentIndex = computed(() => {
    return kanjis.value.findIndex((kanji) => {
      return kanji.character === selected.value?.character;
    });
  });

  watchImmediate(folder, load);

  watchImmediate(kanjis, (_kanjis) => {
    const char = selected.value?.character;
    if (char && _kanjis.every((kanji) => kanji.character !== char)) {
      selected.value = null;
    }

    selected.value ??= _kanjis.at(0);
  });

  async function load() {
    await lock(async () => {
      if (folder.value) {
        raw.value = await commands.searchKanji(folder.value);
      }
      else if (raw.value.length > 0) {
        raw.value = [];
      }
    });
  }

  function go(index: number) {
    const size = kanjis.value.length;
    index = ((index % size) + size) % size;
    selected.value = kanjis.value.at(index);
  }

  function next() {
    go(currentIndex.value + 1);
  }

  function previous() {
    go(currentIndex.value - 1);
  }

  async function exportSet() {
    if (folder.value) {
      await commands.exportSet(folder.value);
    }
  }

  return {
    kanjis,
    raw,
    currentIndex,
    loading: locked,
    load,
    next,
    previous,
    exportSet,
  };
}
