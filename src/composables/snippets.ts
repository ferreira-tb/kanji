import { storeToRefs } from 'pinia';
import { asyncRef } from '@tb-dev/vue';
import { searchSnippets } from '@/commands';
import { useKanjiStore } from '@/stores/kanji';

export function useSnippets() {
  const store = useKanjiStore();
  const { folder, selected } = storeToRefs(store);

  const snippets = asyncRef([], async () => {
    let _snippets: Snippet[] = [];
    if (folder.value && selected.value) {
      const char = selected.value.character;
      _snippets = await searchSnippets(folder.value, char);
    }

    return _snippets;
  });

  return {
    snippets: snippets.state,
    loading: snippets.isLoading,
    load: snippets.execute,
  };
}
