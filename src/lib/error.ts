import { isTauri } from '@tauri-apps/api/core';
import { type MessageDialogOptions, message as showMessage } from '@tauri-apps/plugin-dialog';

export function handleError(err: unknown) {
  console.error(err);
  const message = err instanceof Error ? err.message : String(err);
  if (isTauri()) {
    const options: MessageDialogOptions = { title: 'Error', kind: 'error' };
    showMessage(message, options).catch(console.error);
  }
  else {
    // eslint-disable-next-line no-alert
    window.alert(message);
  }
}
