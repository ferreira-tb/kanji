import { storeToRefs } from 'pinia';
import { handleError } from '@/lib/error';
import type { Option } from '@tb-dev/utils';
import { isTauri } from '@tauri-apps/api/core';
import { useSettingsStore } from '@/stores/settings';
import { tryInjectOrElse, useMutex } from '@tb-dev/vue';
import { createQuiz, createQuizAnswer } from '@/commands';
import {
  effectScope,
  type InjectionKey,
  nextTick,
  readonly,
  ref,
  type Ref,
  shallowRef,
  type ShallowRef,
  watch,
} from 'vue';

interface UseQuizReturn {
  current: Readonly<ShallowRef<Option<QuizQuestion>>>;
  active: Readonly<Ref<boolean>>;
  loading: Readonly<Ref<boolean>>;
  chosen: Ref<Option<KanjiChar>>;
  canAnswer: Ref<boolean>;
  start: () => Promise<void>;
  answer: () => Promise<void>;
  next: () => void;
  leave: () => void;
}

const SYMBOL = Symbol() as InjectionKey<UseQuizReturn>;

export function useQuiz(loadSet: () => Promise<void>) {
  return tryInjectOrElse(SYMBOL, () => {
    const scope = effectScope(/* detached */ true);
    const value = scope.run(() => create(loadSet))!;
    return {
      current: value.current,
      active: value.active,
      loading: value.loading,
      chosen: value.chosen,
      canAnswer: value.canAnswer,
      start: value.start,
      answer: value.answer,
      next: value.next,
      leave: value.leave,
    };
  });
}

export function create(loadSet: () => Promise<void>) {
  const quiz = shallowRef<Option<Quiz>>();
  const current = shallowRef<Option<QuizQuestion>>();
  const active = ref(false);

  const chosen = ref<Option<KanjiChar>>();
  const canAnswer = ref(true);

  const settings = useSettingsStore();
  const { baseUrl } = storeToRefs(settings);

  const { locked, ...mutex } = useMutex();

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

  async function answer() {
    if (
      active.value &&
      quiz.value &&
      current.value &&
      chosen.value &&
      canAnswer.value
    ) {
      await mutex.acquire();
      try {
        await createQuizAnswer(current.value.answer, chosen.value);
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
    void nextTick(loadSet);
  }

  return {
    current: current as Readonly<typeof current>,
    active: readonly(active),
    loading: locked,
    chosen,
    canAnswer,
    start,
    answer,
    next,
    leave,
  };
}
