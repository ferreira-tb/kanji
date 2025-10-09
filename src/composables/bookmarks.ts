import { shallowRef } from 'vue';
import { storeToRefs } from 'pinia';
import { useMutex } from '@tb-dev/vue';
import * as commands from '@/commands';
import { handleError } from '@/lib/error';
import { tryOnMounted } from '@vueuse/core';
import { isTauri } from '@tauri-apps/api/core';
import { useSettingsStore } from '@/stores/settings';

export function useBookmarks() {
  const bookmarks = shallowRef<readonly Bookmark[]>([]);
  const { locked, ...mutex } = useMutex();

  const settings = useSettingsStore();
  const { baseUrl } = storeToRefs(settings);

  async function loadBookmarks() {
    try {
      await mutex.acquire();
      if (isTauri() || baseUrl.value) {
        bookmarks.value = await commands.getBookmarks();
      }
      else {
        bookmarks.value = [];
      }
    }
    catch (err) {
      handleError(err);
    }
    finally {
      mutex.release();
    }
  }

  async function removeBookmark(id: BookmarkId) {
    try {
      await mutex.acquire();
      const rows = await commands.removeBookmark(id);
      if (rows > 0) {
        bookmarks.value = bookmarks.value.filter((bookmark) => {
          return bookmark.id !== id;
        });
      }
    }
    catch (err) {
      handleError(err);
    }
    finally {
      mutex.release();
    }
  }

  tryOnMounted(() => void loadBookmarks());

  return {
    bookmarks: bookmarks as Readonly<typeof bookmarks>,
    loading: locked,
    loadBookmarks,
    removeBookmark,
  };
}
