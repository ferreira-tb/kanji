import { watch } from 'vue';
import { storeToRefs } from 'pinia';
import { asyncRef } from '@tb-dev/vue';
import { searchSnippets } from '@/commands';
import { join } from '@tauri-apps/api/path';
import { useKanjiStore } from '@/stores/kanji';
import { useSettingsStore } from '@/stores/settings';

export function useSnippets() {
  const store = useKanjiStore();
  const { folder, currentSource, currentKanji } = storeToRefs(store);

  const settings = useSettingsStore();
  const { snippetLimit, snippetMinLen } = storeToRefs(settings);

  const snippets = asyncRef([], async () => {
    let result: Snippet[] = [];
    if (folder.value && currentKanji.value) {
      let dir = folder.value;
      if (currentSource.value?.name) {
        dir = await join(dir, currentSource.value.name);
      }

      const kanji = currentKanji.value.character;
      result = await searchSnippets({
        dir,
        kanji,
        limit: snippetLimit.value,
        minLen: snippetMinLen.value,
      });
    }

    return result as readonly Snippet[];
  });

  watch([currentKanji, currentSource, snippetLimit, snippetMinLen], () => void snippets.execute());

  return {
    snippets: snippets.state,
    loading: snippets.isLoading,
    load: snippets.execute,
  };
}
