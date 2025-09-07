import { isTauri } from '@tauri-apps/api/core';
import { type MessageDialogOptions, message as showMessage } from '@tauri-apps/plugin-dialog';

export function handleError(err: unknown) {
  console.error(err);
  if (isTauri()) {
    const options: MessageDialogOptions = { title: 'Error', kind: 'error' };
    const message = err instanceof Error ? err.message : String(err);
    showMessage(message, options).catch(console.error);
  }
}
