import { storeToRefs } from 'pinia';
import { useMutex } from '@tb-dev/vue';
import { handleError } from '@/lib/error';
import type { Option } from '@tb-dev/utils';
import { isTauri } from '@tauri-apps/api/core';
import { useSettingsStore } from '@/stores/settings';
import { createQuiz, createQuizAnswer } from '@/commands';
import { onActivated, readonly, ref, shallowRef, watch } from 'vue';

export function useQuiz() {
  const quiz = shallowRef<Option<Quiz>>();
  const current = shallowRef<Option<QuizQuestion>>();

  const active = ref(false);
  const instant = ref(0);

  const settings = useSettingsStore();
  const { baseUrl } = storeToRefs(settings);

  const { locked, ...mutex } = useMutex();

  onActivated(resetTimer);

  watch(active, (isActive) => {
    if (!isActive) {
      quiz.value = null;
      current.value = null;
    }
  });

  async function start(kanjis: readonly KanjiChar[]) {
    if (!active.value && kanjis.length > 0) {
      await mutex.acquire();
      try {
        if (isTauri() || baseUrl.value) {
          quiz.value = await createQuiz(kanjis);
        }
        else {
          quiz.value = null;
        }

        if (quiz.value) {
          current.value = quiz.value.at(0);
          active.value = true;
          resetTimer();
        }
      }
      catch (err) {
        quiz.value = null;
        current.value = null;
        handleError(err);
      }
      finally {
        mutex.release();
      }
    }
  }

  async function answer(chosen: KanjiChar) {
    if (active.value && quiz.value && current.value) {
      const duration = Date.now() - instant.value;
      await mutex.acquire();
      try {
        await createQuizAnswer(current.value.answer, chosen, duration);
        quiz.value = quiz.value.filter((it) => {
          return it.answer !== current.value?.answer;
        });
      }
      catch (err) {
        handleError(err);
      }
      finally {
        mutex.release();
      }
    }

    resetTimer();
  }

  function next() {
    if (quiz.value) {
      if (quiz.value.length > 0) {
        current.value = quiz.value.at(0);
      }
      else {
        leave();
      }
    }
  }

  function leave() {
    active.value = false;
  }

  function resetTimer() {
    instant.value = Date.now();
  }

  return {
    current: current as Readonly<typeof current>,
    active: readonly(active),
    loading: locked,
    start,
    answer,
    next,
    leave,
  };
}
