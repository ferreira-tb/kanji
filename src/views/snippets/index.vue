<script setup lang="ts">
import { go } from '@/router';
import { open } from '@/commands';
import { storeToRefs } from 'pinia';
import { toPixel } from '@tb-dev/utils';
import { useKanjiStore } from '@/stores/kanji';
import { useSnippets } from '@/composables/snippets';
import { useSettingsStore } from '@/stores/settings';
import { handleError, useHeightDiff } from '@tb-dev/vue';
import { Button, Card, Loading } from '@tb-dev/vue-components';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { type DeepReadonly, nextTick, onActivated, useTemplateRef } from 'vue';

const store = useKanjiStore();
const { folder, selected } = storeToRefs(store);

const settings = useSettingsStore();
const { snippets, load, loading } = useSnippets();

const topbar = useTemplateRef('topbarEl');
const contentHeight = useHeightDiff(topbar);

onActivated(async () => {
  try {
    await nextTick();
    await load();
  } catch (err) {
    handleError(err, true);
  }
});

function onContentClick(snippet: DeepReadonly<Snippet>) {
  if (settings.clipboard) {
    writeText(snippet.content).err();
  }
}
</script>

<template>
  <div class="flex size-full flex-col gap-2">
    <div ref="topbarEl" class="flex h-14 w-full items-center justify-end px-6 py-4">
      <div class="flex items-center justify-center gap-2">
        <Button size="sm" :disabled="loading || !folder || !selected" @click="load">
          <span>Reload</span>
        </Button>
        <Button size="sm" variant="secondary" @click="() => go('home')">
          <span>Return</span>
        </Button>
      </div>
    </div>

    <div
      class="flex flex-col gap-4 overflow-x-hidden overflow-y-auto px-6 pb-12"
      :style="{ height: toPixel(contentHeight) }"
    >
      <div v-if="loading" class="size-full">
        <Loading />
      </div>
      <template v-else>
        <Card v-for="snippet of snippets" :key="snippet.id" class="p-2">
          <div class="flex flex-col gap-2">
            <div
              class="text-muted-foreground cursor-pointer text-xs"
              @click="() => open(snippet.source.path)"
            >
              <span>{{ `${snippet.source.path}:L${snippet.source.line}` }}</span>
            </div>
            <div class="cursor-pointer text-lg" @click="() => onContentClick(snippet)">
              <span>{{ snippet.content }}</span>
            </div>
          </div>
        </Card>
      </template>
    </div>
  </div>
</template>
