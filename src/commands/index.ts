import { invoke } from '@tauri-apps/api/core';
import type { nil, Option } from '@tb-dev/utils';

export async function createTrayIcon() {
  return invoke<nil>('create_tray_icon');
}

export function exportSet(src: string) {
  return invoke<nil>('export_set', { src });
}

export async function open(path: string, line: number) {
  return invoke<nil>('open', { path, line });
}

export async function pickFolder() {
  return invoke<Option<string>>('pick_folder');
}

export async function searchKanji(dir: string) {
  return invoke<Kanji[]>('search_kanji', { dir });
}

export async function searchSnippets(dir: string, kanji: KanjiChar) {
  return invoke<Snippet[]>('search_snippets', { dir, kanji });
}

export async function showWindow() {
  return invoke<nil>('show_window');
}
