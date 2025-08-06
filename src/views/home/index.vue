<script setup lang="ts">
import { go } from '@/router';
import { storeToRefs } from 'pinia';
import { toPixel } from '@tb-dev/utils';
import { useHeightDiff } from '@tb-dev/vue';
import Search from '@/components/Search.vue';
import { useKanjiStore } from '@/stores/kanji';
import { useKanjis } from '@/composables/kanji';
import { useSettingsStore } from '@/stores/settings';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { type DeepReadonly, nextTick, useTemplateRef } from 'vue';
import { Button, Card, CardContent } from '@tb-dev/vue-components';

const store = useKanjiStore();
const { folder, search, selected } = storeToRefs(store);

const settings = useSettingsStore();

const topbar = useTemplateRef('topbarEl');
const contentHeight = useHeightDiff(topbar);

const { kanjis, loading, load, exportSet } = useKanjis();

function onCardClick(kanji: DeepReadonly<Kanji>) {
  selected.value = kanji;
  if (settings.clipboard) {
    writeText(kanji.character).err();
  }
}

function onCardDblClick() {
  void nextTick(() => go('snippets'));
}
</script>

<template>
  <div class="flex size-full flex-col gap-2">
    <div ref="topbarEl" class="flex h-14 w-full items-center justify-between gap-4 px-6 py-4">
      <div class="flex items-center justify-center">
        <Search v-model="search" class="hidden lg:block" />
      </div>
      <div class="flex items-center justify-center gap-4">
        <span v-if="folder" class="text-muted-foreground text-xs">
          {{ folder }}
        </span>
        <div class="flex items-center justify-center gap-2">
          <Button
            size="sm"
            :disabled="loading"
            @click="() => store.setFolder()"
          >
            <span>Select Folder</span>
          </Button>
          <Button
            size="sm"
            variant="secondary"
            :disabled="loading || !folder"
            @click="load"
          >
            <span>Reload</span>
          </Button>
          <Button
            size="sm"
            variant="secondary"
            :disabled="!folder"
            @click="exportSet"
          >
            <span>Export</span>
          </Button>
        </div>
      </div>
    </div>
    <div
      class="overflow-x-hidden overflow-y-auto pb-12"
      :style="{ height: toPixel(contentHeight) }"
    >
      <div
        v-if="kanjis.length > 0"
        class="grid grid-cols-4 gap-2 px-4 sm:grid-cols-6 lg:grid-cols-10 2xl:grid-cols-12"
      >
        <Card v-for="kanji of kanjis" :key="kanji.character" class="p-2">
          <CardContent>
            <div
              class="flex cursor-pointer flex-col items-center"
              @click="() => onCardClick(kanji)"
              @dblclick="() => onCardDblClick()"
            >
              <span class="text-3xl font-bold">{{ kanji.character }}</span>
              <span class="text-muted-foreground text-xs">{{ kanji.seen }}</span>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  </div>
</template>
