import { storeToRefs } from 'pinia';
import { handleError } from '@/lib/error';
import type { Option } from '@tb-dev/utils';
import { isTauri } from '@tauri-apps/api/core';
import { useSettingsStore } from '@/stores/settings';
import { asyncRef, tryInjectOrElse, useMutex } from '@tb-dev/vue';
import { createQuiz, createQuizAnswer, getSet } from '@/commands';
import { computed, effectScope, type InjectionKey, readonly, ref, shallowRef, watch } from 'vue';

const SYMBOL = Symbol() as InjectionKey<ReturnType<typeof create>>;

export function useQuiz() {
  return tryInjectOrElse(SYMBOL, () => {
    const scope = effectScope(/* detached */ true);
    const value = scope.run(create)!;
    return {
      set: value.set,
      current: value.current,
      active: value.active,
      isLoading: value.isLoading,
      isLoadingSet: value.isLoadingSet,
      chosen: value.chosen,
      canAnswer: value.canAnswer,
      start: value.start,
      answer: value.answer,
      next: value.next,
      leave: value.leave,
      loadSet: value.loadSet,
    };
  });
}

export function create() {
  const quiz = shallowRef<Option<Quiz>>();
  const current = shallowRef<Option<QuizQuestion>>();
  const active = ref(false);

  const chosen = ref<Option<KanjiChar>>();
  const canAnswer = ref(true);

  const {
    state: set,
    execute: loadSet,
    isLoading: isLoadingSet,
  } = asyncRef(null, getSet);

  const settings = useSettingsStore();
  const { baseUrl } = storeToRefs(settings);

  const { locked, ...mutex } = useMutex();
  const isLoading = computed(() => locked.value || isLoadingSet.value);

  watch(active, (isActive) => {
    if (!isActive) {
      quiz.value = null;
      current.value = null;
      chosen.value = null;
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

        await next();
      }
      catch (err) {
        quiz.value = null;
        current.value = null;
        chosen.value = null;
        handleError(err);
      }
      finally {
        mutex.release();
      }
    }
  }

  async function answer(option: KanjiChar) {
    if (
      active.value &&
      quiz.value &&
      current.value &&
      canAnswer.value
    ) {
      await mutex.acquire();
      try {
        chosen.value = option;
        canAnswer.value = false;
        await createQuizAnswer(current.value.answer, option);
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
  }

  async function next() {
    if (quiz.value) {
      current.value = quiz.value.at(0);
      active.value = Boolean(current.value);
      canAnswer.value = Boolean(current.value);

      if (!current.value) {
        await leave();
      }
    }
  }

  async function leave() {
    active.value = false;
    chosen.value = null;
    await loadSet();
  }

  return {
    set: set as Readonly<typeof set>,
    current: current as Readonly<typeof current>,
    active: readonly(active),
    isLoading,
    isLoadingSet,
    chosen: readonly(chosen),
    canAnswer: readonly(canAnswer),
    start,
    answer,
    next,
    leave,
    loadSet,
  };
}
