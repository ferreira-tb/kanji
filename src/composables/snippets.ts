import { storeToRefs } from 'pinia';
import { type Ref, watch } from 'vue';
import { asyncRef } from '@tb-dev/vue';
import { searchSnippets } from '@/commands';
import { join } from '@tauri-apps/api/path';
import { useKanjiStore } from '@/stores/kanji';

export function useSnippets(limit: Ref<number>) {
  const store = useKanjiStore();
  const { folder, currentSource, currentKanji } = storeToRefs(store);

  const snippets = asyncRef([], async () => {
    let result: Snippet[] = [];
    if (folder.value && currentKanji.value) {
      let dir = folder.value;
      if (currentSource.value?.name) {
        dir = await join(dir, currentSource.value.name);
      }

      const kanji = currentKanji.value.character;
      result = await searchSnippets({ dir, kanji, limit: limit.value });
    }

    return result as readonly Snippet[];
  });

  watch([currentKanji, currentSource, limit], () => void snippets.execute());

  return {
    snippets: snippets.state,
    loading: snippets.isLoading,
    load: snippets.execute,
  };
}
