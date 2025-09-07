<script setup lang="ts">
import { go } from '@/router';
import { storeToRefs } from 'pinia';
import { useRoute } from 'vue-router';
import * as commands from '@/commands';
import { toPixel } from '@tb-dev/utils';
import { capitalCase } from 'change-case';
import { formatPercent } from '@/lib/intl';
import { useKanjiStore } from '@/stores/kanji';
import { isTauri } from '@tauri-apps/api/core';
import { useKanjis } from '@/composables/kanji';
import { exit } from '@tauri-apps/plugin-process';
import { useRanking } from '@/composables/ranking';
import { useColorMode, useToggle } from '@vueuse/core';
import { handleError, onKeyDown, useHeight } from '@tb-dev/vue';
import { computed, onBeforeMount, onMounted, useTemplateRef } from 'vue';
import {
  Badge,
  Button,
  cn,
  ScrollArea,
  Sidebar,
  SidebarContent,
  SidebarHeader,
  SidebarProvider,
} from '@tb-dev/vue-components';

const store = useKanjiStore();
const { currentKanji, currentSource } = storeToRefs(store);
const ranking = useRanking(currentKanji);

const { kanjis, loading, load, next, previous } = useKanjis();

const route = useRoute();
const [isSidebarOpen] = useToggle(true);

const contentEl = useTemplateRef('content');
const contentHeight = useHeight(contentEl);
const actionEl = useTemplateRef('action');
const actionHeight = useHeight(actionEl);

const listHeight = computed(() => contentHeight.value - actionHeight.value);

useColorMode({
  initialValue: 'dark',
  onError: handleError,
  writeDefaults: true,
});

onKeyDown('F1', () => go('home'));
onKeyDown('F2', () => go('snippets'));
onKeyDown('F3', () => go('quiz'));
onKeyDown('F4', () => go('sources'));
onKeyDown('F7', () => go('settings'));

if (isTauri()) {
  onKeyDown('Escape', () => exit(0).err());
}

onBeforeMount(load);

onMounted(async () => {
  try {
    await commands.createTrayIcon();
    await commands.showWindow();
  }
  catch (err) {
    handleError(err);
  }
});

function setCurrentSource(source: KanjiStatsSource) {
  currentSource.value = source;
  if (route.name !== ('snippets' satisfies Route)) {
    go('snippets');
  }
}
</script>

<template>
  <SidebarProvider v-model:open="isSidebarOpen" style="--sidebar-width: 20rem">
    <Sidebar>
      <SidebarHeader class="pb-0">
        <div v-if="currentKanji" class="flex flex-col gap-4">
          <div class="flex flex-col items-center justify-center gap-4 pt-2">
            <span class="text-9xl">{{ currentKanji.character }}</span>
            <Badge>{{ capitalCase(currentKanji.level) }}</Badge>
          </div>
          <div :class="cn('grid gap-4 px-4', currentKanji.quizzes > 0 ? 'grid-cols-4' : 'grid-cols-2')">
            <div class="flex h-16 flex-col items-center justify-center">
              <span class="text-muted-foreground text-sm">Rank</span>
              <span class="text-lg font-semibold">{{ ranking ? ranking : '?' }}</span>
            </div>
            <div class="flex h-16 flex-col items-center justify-center">
              <span class="text-muted-foreground text-sm">Total</span>
              <span class="text-lg font-semibold">{{ currentKanji.seen }}</span>
            </div>
            <template v-if="currentKanji.quizzes > 0">
              <div class="flex h-16 flex-col items-center justify-center">
                <span class="text-muted-foreground text-sm">Quiz</span>
                <span class="text-lg font-semibold">{{ currentKanji.quizzes }}</span>
              </div>
              <div
                v-if="Number.isFinite(currentKanji.quizAccuracy)"
                class="flex h-16 flex-col items-center justify-center"
              >
                <span class="text-muted-foreground text-sm">Accuracy</span>
                <span class="text-lg font-semibold">
                  {{ formatPercent(currentKanji.quizAccuracy) }}
                </span>
              </div>
            </template>
          </div>
        </div>
      </SidebarHeader>

      <SidebarContent>
        <div
          v-if="currentKanji"
          ref="content"
          class="flex size-full flex-col justify-between gap-6 p-4 select-none"
        >
          <ScrollArea :style="{ height: toPixel(listHeight - 50) }">
            <div id="source-grid" class="text-sidebar-accent-foreground text-sm pr-4">
              <template v-for="source of currentKanji.sources" :key="source.id">
                <div class="ellipsis cursor-pointer" @click="() => setCurrentSource(source)">
                  {{ source.name }}
                </div>
                <div class="text-end">{{ source.seen }}</div>
              </template>
            </div>
          </ScrollArea>

          <div ref="action" class="grid grid-cols-2 items-center gap-4">
            <Button
              variant="secondary"
              size="sm"
              :disabled="loading || kanjis.length === 0"
              @click="previous"
            >
              <span>Previous</span>
            </Button>
            <Button
              variant="secondary"
              size="sm"
              :disabled="loading || kanjis.length === 0"
              @click="next"
            >
              <span>Next</span>
            </Button>
          </div>
        </div>
      </SidebarContent>
    </Sidebar>

    <main class="h-screen w-full select-none">
      <div class="size-full overflow-hidden p-0">
        <RouterView #default="{ Component }">
          <template v-if="Component">
            <KeepAlive>
              <component :is="Component" />
            </KeepAlive>
          </template>
        </RouterView>
      </div>
    </main>
  </SidebarProvider>
</template>

<style scoped>
#source-grid {
  display: grid;
  grid-template-columns: auto 1fr;
  column-gap: 1rem;
}
</style>
