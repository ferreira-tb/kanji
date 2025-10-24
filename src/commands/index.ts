import * as api from '@/api';
import { clamp } from 'es-toolkit';
import { invoke } from '@tauri-apps/api/core';

export async function createBookmark(snippet: Snippet) {
  if (__DESKTOP__) {
    return invoke<BookmarkId>('create_bookmark', { snippet });
  }
  else {
    return api.createBookmark(snippet);
  }
}

export async function createQuiz(kind: QuizKind) {
  if (__DESKTOP__) {
    return invoke<Quiz>('create_quiz', { kind });
  }
  else {
    return api.createQuiz(kind);
  }
}

export async function createQuizAnswer(question: KanjiChar, answer: KanjiChar, source: SourceId) {
  if (__DESKTOP__) {
    return invoke<QuizAnswerId>('create_quiz_answer', { question, answer, source });
  }
  else {
    return api.createQuizAnswer(question, answer, source);
  }
}

export async function createSource(source?: Option<string | string[]>) {
  if (__DESKTOP__) {
    source ??= await pickFolders();
    if (Array.isArray(source)) {
      await Promise.all(source.map(createSource));
    }
    else {
      await invoke<SourceId>('create_source', { source });
    }
  }
  else {
    await api.createSource(source);
  }
}

export async function createTrayIcon() {
  if (__DESKTOP__) {
    await invoke('create_tray_icon');
  }
}

export async function exportSet() {
  if (__DESKTOP__) {
    await invoke('export_set');
  }
}

export async function getBookmarks() {
  if (__DESKTOP__) {
    return invoke<readonly Bookmark[]>('get_bookmarks');
  }
  else {
    return api.getBookmarks();
  }
}

export async function getQuizAnswers() {
  if (__DESKTOP__) {
    return invoke<readonly QuizAnswer[]>('get_quiz_answers');
  }
  else {
    return api.getQuizAnswers();
  }
}

export async function getServerAddr() {
  if (__DESKTOP__) {
    return invoke<string>('get_server_addr');
  }
  else {
    return null;
  }
}

export async function getSet() {
  if (__DESKTOP__) {
    return invoke<KanjiSet>('get_set');
  }
  else {
    return api.getSet();
  }
}

export async function getSources() {
  if (__DESKTOP__) {
    return invoke<readonly Source[]>('get_sources');
  }
  else {
    return api.getSources();
  }
}

export async function open(path: string, line: number) {
  if (__DESKTOP__) {
    await invoke('open', { path, line });
  }
}

export async function pickFolders() {
  if (__DESKTOP__) {
    return invoke<string[]>('pick_folders');
  }
  else {
    return [];
  }
}

export async function renameSource(id: SourceId, name: string) {
  if (__DESKTOP__) {
    await invoke('rename_source', { id, name });
  }
  else {
    await api.renameSource(id, name);
  }
}

export async function removeBookmark(id: BookmarkId) {
  if (__DESKTOP__) {
    return invoke<number>('remove_bookmark', { id });
  }
  else {
    return api.removeBookmark(id);
  }
}

export async function removeSource(id: SourceId) {
  if (__DESKTOP__) {
    return invoke<number>('remove_source', { id });
  }
  else {
    return api.removeSource(id);
  }
}

export async function searchKanji() {
  if (__DESKTOP__) {
    return invoke<KanjiStats[]>('search_kanji');
  }
  else {
    return api.searchKanji();
  }
}

export async function searchSnippets(kanji: KanjiChar, source?: Option<SourceId>) {
  source ??= null;
  if (__DESKTOP__) {
    return invoke<Snippet[]>('search_snippets', { kanji, source });
  }
  else {
    return api.searchSnippets(kanji, source);
  }
}

export async function setSourceWeight(id: SourceId, weight: SourceWeight) {
  weight = clamp(Math.trunc(weight), 1, 10);
  if (__DESKTOP__) {
    await invoke('set_source_weight', { id, weight });
  }
  else {
    await api.setSourceWeight(id, weight);
  }
}

export async function showWindow() {
  if (__DESKTOP__) {
    await invoke('show_window');
  }
}

export async function toggleSource(id: SourceId, enabled: boolean) {
  if (__DESKTOP__) {
    await invoke('toggle_source', { id, enabled });
  }
  else {
    await api.toggleSource(id, enabled);
  }
}
