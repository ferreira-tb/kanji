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
  type MaybeRefOrGetter,
  nextTick,
  readonly,
  ref,
  type Ref,
  shallowRef,
  type ShallowRef,
  toValue,
  watch,
} from 'vue';

interface UseQuizReturn {
  current: Readonly<ShallowRef<Option<QuizQuestion>>>;
  active: Readonly<Ref<boolean>>;
  loading: Readonly<Ref<boolean>>;
  chosen: Readonly<Ref<Option<KanjiChar>>>;
  canAnswer: Readonly<Ref<boolean>>;
  start: () => Promise<void>;
  answer: () => Promise<void>;
  next: () => void;
  leave: () => void;
}

const SYMBOL = Symbol() as InjectionKey<UseQuizReturn>;

export function useQuiz(loadSet: MaybeRefOrGetter<() => Promise<void>>) {
  const fn = toValue(loadSet);
  return tryInjectOrElse(SYMBOL, () => {
    const scope = effectScope(/* detached */ true);
    const value = scope.run(() => create(fn))!;
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

        next();
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

  function next() {
    if (quiz.value) {
      current.value = quiz.value.at(0);
      active.value = Boolean(current.value);
      canAnswer.value = Boolean(current.value);

      if (!current.value) {
        leave();
      }
    }
  }

  function leave() {
    active.value = false;
    chosen.value = null;
    void nextTick(loadSet);
  }

  return {
    current: current as Readonly<typeof current>,
    active: readonly(active),
    loading: locked,
    chosen: readonly(chosen),
    canAnswer: readonly(canAnswer),
    start,
    answer,
    next,
    leave,
  };
}
