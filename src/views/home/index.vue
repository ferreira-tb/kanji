<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { toPixel } from '@tb-dev/utils';
import type { Kanji } from '@/api/bindings';
import Search from '@/components/Search.vue';
import { useKanjiStore } from '@/stores/kanji';
import { useKanjis } from '@/composables/kanji';
import { type DeepReadonly, useTemplateRef } from 'vue';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { Button, Card, handleError, useHeightDiff } from '@tb-dev/vue';

const store = useKanjiStore();
const { folder, search, selected } = storeToRefs(store);

const topbar = useTemplateRef('topbarEl');
const contentHeight = useHeightDiff(topbar);

const { kanjis, loading } = useKanjis();

async function onCardClick(kanji: DeepReadonly<Kanji>) {
  try {
    selected.value = kanji;
    await writeText(kanji.character);
  } catch (err) {
    handleError(err);
  }
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
        <div class="flex items-center justify-center">
          <Button size="sm" :disabled="loading" @click="() => store.pickFolder()">
            Select Folder
          </Button>
        </div>
      </div>
    </div>
    <div
      class="h-full overflow-x-hidden overflow-y-auto pb-12"
      :style="{ height: toPixel(contentHeight) }"
    >
      <div
        v-if="kanjis.length > 0"
        class="grid gap-2 px-4 sm:grid-cols-8 lg:grid-cols-10 2xl:grid-cols-12"
      >
        <Card v-for="kanji of kanjis" :key="kanji.character" class="p-2">
          <div class="flex cursor-pointer flex-col items-center" @click="() => onCardClick(kanji)">
            <span class="text-3xl font-bold">{{ kanji.character }}</span>
            <span class="text-muted-foreground text-xs">{{ kanji.seen }}</span>
          </div>
        </Card>
      </div>
    </div>
  </div>
</template>
