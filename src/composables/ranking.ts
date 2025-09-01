import { useKanjis } from './kanji';
import type { Option } from '@tb-dev/utils';
import { computed, type DeepReadonly, type Ref } from 'vue';

export function useRanking(kanji: Ref<Option<DeepReadonly<KanjiStats>>>) {
  const { raw } = useKanjis();
  return computed(() => {
    const index = raw.value
      .toSorted(({ seen: a }, { seen: b }) => b - a)
      .findIndex(({ character }) => {
        return kanji.value?.character === character;
      });

    return index !== -1 ? index + 1 : null;
  });
}
