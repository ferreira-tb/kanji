import { invoke } from '@tauri-apps/api/core';
import type { nil, Option } from '@tb-dev/utils';

export async function createSource(source?: Option<string | string[]>) {
  source ??= await pickFolders();
  if (Array.isArray(source)) {
    await Promise.all(source.map(createSource));
  }
  else {
    await invoke<SourceId>('create_source', { source });
  }
}

export async function createTrayIcon() {
  return invoke<nil>('create_tray_icon');
}

export function exportSet() {
  return invoke<nil>('export_set');
}

export async function open(path: string, line: number) {
  return invoke<nil>('open', { path, line });
}

export async function pickFolders() {
  return invoke<string[]>('pick_folders');
}

export async function searchKanji() {
  return invoke<KanjiStats[]>('search_kanji');
}

export async function searchSnippets(kanji: KanjiChar, source?: Option<SourceId>) {
  source ??= null;
  return invoke<Snippet[]>('search_snippets', { kanji, source });
}

export async function showWindow() {
  return invoke<nil>('show_window');
}
