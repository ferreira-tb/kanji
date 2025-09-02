import { check } from '@tauri-apps/plugin-updater';
import { message } from '@tauri-apps/plugin-dialog';
import { relaunch } from '@tauri-apps/plugin-process';

export async function checkForUpdates() {
  if (!__DEBUG_ASSERTIONS__) {
    const update = await check();
    if (update) {
      const result = await message(`New version available: ${update.version}.`, {
        title: `Kanji ${update.currentVersion}`,
        buttons: { ok: 'Update', cancel: 'Cancel' },
      });

      if (result === 'Update') {
        await update.downloadAndInstall();
        await relaunch();
      }
    }
  }
}
