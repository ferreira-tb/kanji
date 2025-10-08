import { storeToRefs } from 'pinia';
import * as commands from '@/commands';
import { handleError } from '@/lib/error';
import { isTauri } from '@tauri-apps/api/core';
import { useSettingsStore } from '@/stores/settings';
import { asyncRef, tryInjectOrElse, useMutex } from '@tb-dev/vue';
import { computed, effectScope, type InjectionKey, readonly, ref, shallowRef, watch } from 'vue';

const SYMBOL = Symbol() as InjectionKey<ReturnType<typeof create>>;

export function useQuiz() {
  return tryInjectOrElse(SYMBOL, () => {
    const scope = effectScope(/* detached */ true);
    const value = scope.run(create)!;
    return {
      set: value.set,
      quizSize: value.quizSize,
      active: value.active,
      isLoading: value.isLoading,
      isLoadingSet: value.isLoadingSet,
      currentQuestion: value.currentQuestion,
      currentIndex: value.currentIndex,
      currentSnippet: value.currentSnippet,
      currentBookmark: value.currentBookmark,
      chosenAnswer: value.chosenAnswer,
      canAnswer: value.canAnswer,
      start: value.start,
      startRandom: value.startRandom,
      startWith: value.startWith,
      answer: value.answer,
      next: value.next,
      leave: value.leave,
      loadSet: value.loadSet,
      createBookmark: value.createBookmark,
      removeBookmark: value.removeBookmark,
    };
  });
}

export function create() {
  const quiz = shallowRef<Option<Quiz>>();
  const quizSize = ref<Option<number>>();
  const active = ref(false);

  const currentQuestion = ref<Option<QuizQuestion>>();
  const currentIndex = ref<Option<number>>();
  const currentSnippet = computed(() => currentQuestion.value?.snippet);
  const currentBookmark = computed(() => currentSnippet.value?.bookmark);

  const chosenAnswer = ref<Option<KanjiChar>>();
  const canAnswer = ref(true);

  const {
    state: set,
    execute: loadSet,
    isLoading: isLoadingSet,
  } = asyncRef(null, commands.getSet);

  const settings = useSettingsStore();
  const { baseUrl } = storeToRefs(settings);

  const { locked, ...mutex } = useMutex();
  const isLoading = computed(() => locked.value || isLoadingSet.value);

  watch(active, (isActive) => {
    if (!isActive) {
      cleanup();
    }
  });

  async function startWith(f: () => Promise<Quiz>) {
    if (!active.value) {
      await mutex.acquire();
      try {
        if (isTauri() || baseUrl.value) {
          quiz.value = await f();
          quizSize.value = quiz.value.length;
        }
        else {
          quiz.value = null;
          quizSize.value = null;
        }

        await next();
      }
      catch (err) {
        void leave();
        handleError(err);
      }
      finally {
        mutex.release();
      }
    }
  }

  async function start(kanjis: readonly KanjiChar[]) {
    if (kanjis.length > 0) {
      await startWith(() => commands.createQuiz(kanjis));
    }
  }

  function startRandom() {
    return startWith(commands.createRandomQuiz);
  }

  async function answer(option: KanjiChar) {
    if (
      active.value &&
      quiz.value &&
      currentQuestion.value &&
      canAnswer.value
    ) {
      await mutex.acquire();
      try {
        chosenAnswer.value = option;
        canAnswer.value = false;
        await commands.createQuizAnswer(currentQuestion.value.answer, option);
        quiz.value = quiz.value.filter((it) => {
          return it.answer !== currentQuestion.value?.answer;
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
    const total = quizSize.value ?? 0;
    const pending = quiz.value?.length ?? 0;
    if (quiz.value && pending > 0 && total > 0) {
      currentQuestion.value = quiz.value.at(0);
      active.value = Boolean(currentQuestion.value);
      canAnswer.value = Boolean(currentQuestion.value);

      if (currentQuestion.value) {
        currentIndex.value = Math.min(total, (total - pending) + 1);
      }
    }
    else {
      await leave();
    }
  }

  async function leave() {
    active.value = false;
    await loadSet();
  }

  function cleanup() {
    quiz.value = null;
    quizSize.value = null;
    currentQuestion.value = null;
    currentIndex.value = null;
    chosenAnswer.value = null;
    canAnswer.value = false;
  }

  async function createBookmark() {
    if (currentQuestion.value && currentSnippet.value && !currentBookmark.value) {
      await mutex.acquire();
      try {
        const snippetId = currentSnippet.value.id;
        const bookmarkId = await commands.createBookmark(currentSnippet.value);
        if (currentQuestion.value.snippet.id === snippetId) {
          currentQuestion.value.snippet.bookmark = bookmarkId;
        }
      }
      catch (err) {
        handleError(err);
      }
      finally {
        mutex.release();
      }
    }
  }

  async function removeBookmark() {
    if (currentQuestion.value && currentSnippet.value && currentBookmark.value) {
      await mutex.acquire();
      try {
        const snippetId = currentSnippet.value.id;
        const rows = await commands.removeBookmark(currentBookmark.value);
        if (rows > 0 && currentQuestion.value.snippet.id === snippetId) {
          currentQuestion.value.snippet.bookmark = null;
        }
      }
      catch (err) {
        handleError(err);
      }
      finally {
        mutex.release();
      }
    }
  }

  return {
    set: set as Readonly<typeof set>,
    quizSize: readonly(quizSize),
    active: readonly(active),
    isLoading,
    isLoadingSet,
    currentQuestion: currentQuestion as Readonly<typeof currentQuestion>,
    currentSnippet,
    currentBookmark,
    currentIndex: readonly(currentIndex),
    chosenAnswer: readonly(chosenAnswer),
    canAnswer: readonly(canAnswer),
    start,
    startRandom,
    startWith,
    answer,
    next,
    leave,
    loadSet,
    createBookmark,
    removeBookmark,
  };
}
