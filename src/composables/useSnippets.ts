import { watch } from 'vue';
import { storeToRefs } from 'pinia';
import { asyncRef } from '@tb-dev/vue';
import { searchSnippets } from '@/commands';
import { useKanjiStore } from '@/stores/kanji';
import { useSettingsStore } from '@/stores/settings';

export function useSnippets() {
  const store = useKanjiStore();
  const { currentSource, currentKanji } = storeToRefs(store);

  const settings = useSettingsStore();
  const { baseUrl, snippetLimit, snippetMinLen } = storeToRefs(settings);

  const snippets = asyncRef([], async () => {
    let result: Snippet[] = [];
    if (currentKanji.value && (__DESKTOP__ || baseUrl.value)) {
      const kanji = currentKanji.value.character;
      result = await searchSnippets(kanji, currentSource.value?.id);
    }

    return result as readonly Snippet[];
  });

  watch([
    baseUrl,
    currentKanji,
    currentSource,
    snippetLimit,
    snippetMinLen,
  ], () => void snippets.load());

  return {
    snippets: snippets.state,
    loading: snippets.loading,
    load: snippets.load,
  };
}
