import { message as showMessage } from '@tauri-apps/plugin-dialog';

export function handleError(err: unknown) {
  console.error(err);
  const message = err instanceof Error ? err.message : String(err);
  void showMessage(message, { title: 'Error', kind: 'error' });
}
