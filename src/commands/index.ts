import { invoke } from '@tauri-apps/api/core';
import type { nil, Option } from '@tb-dev/utils';

export async function createTrayIcon() {
  return invoke<nil>('create_tray_icon');
}

export async function pickFolder() {
  return invoke<Option<string>>('pick_folder');
}

export async function searchKanji(path: string) {
  return invoke<Kanji[]>('search_kanji', { path });
}

export async function showWindow() {
  return invoke<nil>('show_window');
}
