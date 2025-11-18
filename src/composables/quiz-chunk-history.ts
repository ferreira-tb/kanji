import { shallowRef } from 'vue';
import { storeToRefs } from 'pinia';
import { useMutex } from '@tb-dev/vue';
import * as commands from '@/commands';
import { handleError } from '@/lib/error';
import { useSettingsStore } from '@/stores/settings';

type QuizChunkHistory = ReadonlyMap<KanjiSetChunkId, QuizChunkHistoryEntry>;

export function useQuizChunkHistory() {
  const history = shallowRef<QuizChunkHistory>(new Map());
  const { locked, ...mutex } = useMutex();

  const settings = useSettingsStore();
  const { baseUrl } = storeToRefs(settings);

  async function loadQuizChunkHistoryEntries() {
    try {
      await mutex.acquire();
      if (__DESKTOP__ || baseUrl.value) {
        history.value = toMap(await commands.getQuizChunkHistoryEntries());
      }
      else {
        history.value = new Map();
      }
    }
    catch (err) {
      handleError(err);
    }
    finally {
      mutex.release();
    }
  }

  return {
    quizChunkHistory: history,
    loading: locked,
    loadQuizChunkHistoryEntries,
  };
}

function toMap(entries: readonly QuizChunkHistoryEntry[]) {
  const map = new Map<KanjiSetChunkId, QuizChunkHistoryEntry>();
  entries.forEach((entry) => map.set(entry.id, entry));
  return map;
}
