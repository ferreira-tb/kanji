<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { toPixel } from '@tb-dev/utils';
import Search from '@/components/Search.vue';
import { toValue, useTemplateRef } from 'vue';
import { useSettingsStore } from '@/stores/settings';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { commands, type Frequency, type Kanji } from '@/api/bindings';
import { asyncComputed, Button, Card, handleError, useHeightDiff } from '@tb-dev/vue';

const settings = useSettingsStore();
const { folder, sorting, search } = storeToRefs(settings);

const topbar = useTemplateRef('topbarEl');
const contentHeight = useHeightDiff(topbar);

const frequency = asyncComputed<readonly Frequency[]>([], async () => {
  if (folder.value) {
    // They must be captured before the await point.
    const _search = toValue(search);
    const _sorting = toValue(sorting);

    let result = await commands.searchKanji(folder.value);
    if (typeof _search === 'string' && _search.length > 0) {
      result = result.filter(({ kanji }) => {
        return _search.includes(kanji.character);
      });
    }

    result.sort(({ amount: a }, { amount: b }) => {
      return _sorting.ascending ? a - b : b - a;
    });

    return result;
  }

  return [];
});

async function onCardClick(kanji: Kanji) {
  try {
    await writeText(kanji.character);
  } catch (err) {
    handleError(err);
  }
}
</script>

<template>
  <div class="flex size-full flex-col gap-2">
    <div ref="topbarEl" class="flex h-12 w-full items-center justify-between gap-4 px-6 pt-4">
      <div class="flex items-center justify-center">
        <Search v-model="search" />
      </div>
      <div class="flex items-center justify-center gap-4">
        <span v-if="folder" class="text-muted-foreground text-xs">
          {{ folder }}
        </span>
        <div class="flex items-center justify-center">
          <Button size="sm" @click="() => settings.pickFolder()">Select Folder</Button>
        </div>
      </div>
    </div>
    <div
      class="h-full overflow-x-hidden overflow-y-auto pb-12"
      :style="{ height: toPixel(contentHeight) }"
    >
      <div
        v-if="frequency.length > 0"
        class="grid gap-2 px-4 sm:grid-cols-8 lg:grid-cols-10 2xl:grid-cols-12"
      >
        <Card v-for="{ kanji, amount } of frequency" :key="kanji.character" class="p-2">
          <div class="flex cursor-pointer flex-col items-center" @click="() => onCardClick(kanji)">
            <span class="text-3xl font-bold">{{ kanji.character }}</span>
            <span class="text-muted-foreground text-xs">{{ amount }}</span>
          </div>
        </Card>
      </div>
    </div>
  </div>
</template>
