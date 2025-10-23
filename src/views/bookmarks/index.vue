<script setup lang="ts">
import { useTemplateRef } from 'vue';
import { toPixel } from '@tb-dev/utils';
import { useHeightDiff } from '@tb-dev/vue';
import { useSources } from '@/composables/sources';
import { useSettingsStore } from '@/stores/settings';
import { useBookmarks } from '@/composables/bookmarks';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { Button, Card, CardContent, Loading, SidebarTrigger } from '@tb-dev/vue-components';

const settings = useSettingsStore();

const {
  bookmarks,
  loading,
  loadBookmarks,
  removeBookmark,
} = useBookmarks();

const { findSource } = useSources();

const topbar = useTemplateRef('topbarEl');
const contentHeight = useHeightDiff(topbar);

function onContentClick(bookmark: Bookmark) {
  if (settings.clipboard) {
    writeText(bookmark.snippet).err();
  }
}
</script>

<template>
  <div class="flex size-full flex-col gap-2">
    <div ref="topbarEl" class="flex h-14 w-full items-center justify-between px-2 md:px-6 py-4">
      <SidebarTrigger />

      <div class="flex items-center justify-center gap-2">
        <Button size="sm" :disabled="loading" @click="loadBookmarks">
          <span>Reload</span>
        </Button>
      </div>
    </div>

    <div
      class="flex flex-col gap-2 md:gap-4 overflow-x-hidden overflow-y-auto px-1 md:px-6 pb-safe-12"
      :style="{ height: toPixel(contentHeight) }"
    >
      <div v-if="loading" class="size-full">
        <Loading />
      </div>
      <template v-else>
        <Card v-for="bookmark of bookmarks" :key="bookmark.id" class="p-0">
          <CardContent class="p-2">
            <div class="flex justify-between items-center gap-4">
              <div class="flex flex-col gap-2">
                <div class="text-muted-foreground text-xs">
                  <span>{{ findSource(bookmark.sourceId)?.name ?? '' }}</span>
                </div>
                <div class="cursor-pointer text-lg" @click="() => onContentClick(bookmark)">
                  <span>{{ bookmark.snippet }}</span>
                </div>
              </div>

              <div class="flex justify-center items-center gap-2">
                <Button variant="destructive" size="sm" @click="() => removeBookmark(bookmark.id)">
                  <span>Remove</span>
                </Button>
              </div>
            </div>
          </CardContent>
        </Card>
      </template>
    </div>
  </div>
</template>
