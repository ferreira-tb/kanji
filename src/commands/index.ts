import * as api from '@/lib/api';
import { clamp } from 'es-toolkit';
import type { nil, Option } from '@tb-dev/utils';
import { invoke, isTauri } from '@tauri-apps/api/core';

export async function createQuiz(kanjis: readonly KanjiChar[]) {
  if (isTauri()) {
    return invoke<Quiz>('create_quiz', { kanjis });
  }
  else {
    return api.createQuiz(kanjis);
  }
}

export async function createQuizAnswer(question: KanjiChar, answer: KanjiChar) {
  if (isTauri()) {
    return invoke<nil>('create_quiz_answer', { question, answer });
  }
  else {
    return api.createQuizAnswer(question, answer);
  }
}

export async function createSource(source?: Option<string | string[]>) {
  if (isTauri()) {
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
  if (isTauri()) {
    await invoke('create_tray_icon');
  }
}

export async function exportSet() {
  if (isTauri()) {
    await invoke('export_set');
  }
}

export async function getServerAddr() {
  if (isTauri()) {
    return invoke<string>('get_server_addr');
  }
  else {
    return null;
  }
}

export async function getSet() {
  if (isTauri()) {
    return invoke<KanjiSet>('get_set');
  }
  else {
    return api.getSet();
  }
}

export async function getSources() {
  if (isTauri()) {
    return invoke<readonly Source[]>('get_sources');
  }
  else {
    return api.getSources();
  }
}

export async function open(path: string, line: number) {
  if (isTauri()) {
    await invoke('open', { path, line });
  }
}

export async function pickFolders() {
  if (isTauri()) {
    return invoke<string[]>('pick_folders');
  }
  else {
    return [];
  }
}

export async function renameSource(id: SourceId, name: string) {
  if (isTauri()) {
    await invoke('rename_source', { id, name });
  }
  else {
    await api.renameSource(id, name);
  }
}

export async function searchKanji() {
  if (isTauri()) {
    return invoke<KanjiStats[]>('search_kanji');
  }
  else {
    return api.searchKanji();
  }
}

export async function searchSnippets(kanji: KanjiChar, source?: Option<SourceId>) {
  source ??= null;
  if (isTauri()) {
    return invoke<Snippet[]>('search_snippets', { kanji, source });
  }
  else {
    return api.searchSnippets(kanji, source);
  }
}

export async function setSourceWeight(id: SourceId, weight: SourceWeight) {
  weight = clamp(Math.trunc(weight), 1, 5);
  if (isTauri()) {
    await invoke('set_source_weight', { id, weight });
  }
  else {
    await api.setSourceWeight(id, weight);
  }
}

export async function showWindow() {
  if (isTauri()) {
    await invoke('show_window');
  }
}

export async function toggleSource(id: SourceId, enabled: boolean) {
  if (isTauri()) {
    await invoke('toggle_source', { id, enabled });
  }
  else {
    await api.toggleSource(id, enabled);
  }
}
