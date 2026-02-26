import * as api from '@/api';
import { clamp } from 'es-toolkit';
import { handleError } from '@/lib/error';
import { invoke } from '@tauri-apps/api/core';

export async function clearQuizChunkHistory() {
  if (__DESKTOP__) {
    return invoke<number>('clear_quiz_chunk_history');
  }
  else {
    return api.clearQuizChunkHistory();
  }
}

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

export async function createQuizChunkHistoryEntry(id: KanjiSetChunkId) {
  if (__DESKTOP__) {
    await invoke('create_quiz_chunk_history_entry', { id });
  }
  else {
    await api.createQuizChunkHistoryEntry(id);
  }
}

export async function createSource(source?: Option<string | string[]>) {
  if (__DESKTOP__) {
    source ??= await pickFolders();
    if (Array.isArray(source)) {
      for (const result of await Promise.allSettled(source.map(createSource))) {
        if (result.status === 'rejected') {
          handleError(result.reason);
        }
      }
    }
    else {
      await invoke<SourceId>('create_source', { source });
    }
  }
  else {
    await api.createSource(source);
  }
}

export async function createSourceGroup(name: string) {
  if (__DESKTOP__) {
    return invoke<SourceGroupId>('create_source_group', { name });
  }
  else {
    return api.createSourceGroup(name);
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

export async function getQuizChunkHistoryEntries() {
  if (__DESKTOP__) {
    return invoke<readonly QuizChunkHistoryEntry[]>('get_quiz_chunk_history_entries');
  }
  else {
    return api.getQuizChunkHistoryEntries();
  }
}

export async function getQuizSourceStats() {
  if (__DESKTOP__) {
    return invoke<readonly QuizSourceStats[]>('get_quiz_source_stats');
  }
  else {
    return api.getQuizSourceStats();
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

export async function getSource(id: SourceId) {
  if (__DESKTOP__) {
    return invoke<Source>('get_source');
  }
  else {
    return api.getSource(id);
  }
}

export async function getSourceGroup(id: SourceGroupId) {
  if (__DESKTOP__) {
    return invoke<SourceGroup>('get_source_group');
  }
  else {
    return api.getSourceGroup(id);
  }
}

export async function getSourceGroupIds() {
  if (__DESKTOP__) {
    return invoke<readonly SourceGroupId[]>('get_source_group_ids');
  }
  else {
    return api.getSourceGroupIds();
  }
}

export async function getSourceGroupSourceIds(id: SourceGroupId) {
  if (__DESKTOP__) {
    return invoke<SourceId[]>('get_source_group_source_ids', { id });
  }
  else {
    return api.getSourceGroupSourceIds(id);
  }
}

export async function getSourceGroupSources(id: SourceGroupId) {
  if (__DESKTOP__) {
    return invoke<readonly Source[]>('get_source_group_sources', { id });
  }
  else {
    return api.getSourceGroupSources(id);
  }
}

export async function getSourceGroups() {
  if (__DESKTOP__) {
    return invoke<readonly SourceGroup[]>('get_source_groups');
  }
  else {
    return api.getSourceGroups();
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

export async function openEditor(path: string, line: number) {
  if (__DESKTOP__) {
    await invoke('open_editor', { path, line });
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

export async function removeSourceGroup(id: SourceGroupId) {
  if (__DESKTOP__) {
    return invoke<number>('remove_source_group', { id });
  }
  else {
    return api.removeSourceGroup(id);
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

export async function renameSourceGroup(id: SourceGroupId, name: string) {
  if (__DESKTOP__) {
    await invoke('rename_source_group', { id, name });
  }
  else {
    await api.renameSourceGroup(id, name);
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

export async function setSourceGroupSources(id: SourceGroupId, sources: readonly SourceId[]) {
  if (__DESKTOP__) {
    await invoke('set_source_group_sources', { id, sources });
  }
  else {
    await api.setSourceGroupSources(id, sources);
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
