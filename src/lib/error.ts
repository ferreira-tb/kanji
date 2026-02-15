import { toast } from '@tb-dev/vue-sonner';
import { error as logError } from '@tauri-apps/plugin-log';

export function handleError(err: unknown) {
  const message = err instanceof Error ? err.message : String(err);
  if (__DEBUG_ASSERTIONS__ && __DESKTOP__) {
    void logError(message);
  }

  toast.error(message, {
    closeButton: false,
    dismissible: true,
    duration: 3000,
  });
}
