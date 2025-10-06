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
      current: value.current,
      active: value.active,
      isLoading: value.isLoading,
      isLoadingSet: value.isLoadingSet,
      chosen: value.chosen,
      canAnswer: value.canAnswer,
      snippet: value.snippet,
      bookmark: value.bookmark,
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
  const current = ref<Option<QuizQuestion>>();
  const active = ref(false);

  const chosen = ref<Option<KanjiChar>>();
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

  const snippet = computed(() => current.value?.snippet);
  const bookmark = computed(() => snippet.value?.bookmark);

  watch(active, (isActive) => {
    if (!isActive) {
      quiz.value = null;
      current.value = null;
      chosen.value = null;
    }
  });

  async function startWith(f: () => Promise<Quiz>) {
    if (!active.value) {
      await mutex.acquire();
      try {
        if (isTauri() || baseUrl.value) {
          quiz.value = await f();
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
      current.value &&
      canAnswer.value
    ) {
      await mutex.acquire();
      try {
        chosen.value = option;
        canAnswer.value = false;
        await commands.createQuizAnswer(current.value.answer, option);
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

  async function createBookmark() {
    if (current.value && snippet.value && !bookmark.value) {
      await mutex.acquire();
      try {
        const snippetId = snippet.value.id;
        const bookmarkId = await commands.createBookmark(snippet.value);
        if (current.value.snippet.id === snippetId) {
          current.value.snippet.bookmark = bookmarkId;
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
    if (current.value && snippet.value && bookmark.value) {
      await mutex.acquire();
      try {
        const snippetId = snippet.value.id;
        const rows = await commands.removeBookmark(bookmark.value);
        if (rows > 0 && current.value.snippet.id === snippetId) {
          current.value.snippet.bookmark = null;
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
    current: current as Readonly<typeof current>,
    active: readonly(active),
    isLoading,
    isLoadingSet,
    chosen: readonly(chosen),
    canAnswer: readonly(canAnswer),
    snippet,
    bookmark,
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
