import { useFrequency } from './frequency';
import type { Option } from '@tb-dev/utils';
import type { Frequency } from '@/api/bindings';
import { computed, type DeepReadonly, type Ref } from 'vue';

export function useRanking(frequency: Ref<Option<DeepReadonly<Frequency>>>) {
  const { raw } = useFrequency();
  return computed(() => {
    const index = raw.value.findIndex((it) => {
      return it.kanji.character === frequency.value?.kanji.character;
    });

    return index !== -1 ? index + 1 : null;
  });
}
