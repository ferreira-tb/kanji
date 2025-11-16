import { error as logError } from '@tauri-apps/plugin-log';
import { message as showMessage } from '@tauri-apps/plugin-dialog';

export function handleError(err: unknown) {
  const message = err instanceof Error ? err.message : String(err);
  if (__DEBUG_ASSERTIONS__) {
    void logError(message);
  }

  void showMessage(message, { title: 'Error', kind: 'error' });
}
