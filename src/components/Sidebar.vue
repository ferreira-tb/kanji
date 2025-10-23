<script setup lang="ts">
import { go } from '@/router';
import { storeToRefs } from 'pinia';
import { useRoute } from 'vue-router';
import { toPixel } from '@tb-dev/utils';
import { useHeight } from '@tb-dev/vue';
import { capitalCase } from 'change-case';
import { formatPercent } from '@/lib/intl';
import { useKanjiStore } from '@/stores/kanji';
import { ChevronUpIcon } from 'lucide-vue-next';
import { useKanjis } from '@/composables/kanji';
import { useRanking } from '@/composables/ranking';
import { computed, onBeforeMount, useTemplateRef } from 'vue';
import {
  Badge,
  Button,
  cn,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
  ScrollArea,
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
} from '@tb-dev/vue-components';

const store = useKanjiStore();
const { currentKanji, currentSource } = storeToRefs(store);
const ranking = useRanking(currentKanji);

const { kanjis, loading, load, next, previous } = useKanjis();

const route = useRoute();

const contentEl = useTemplateRef('content');
const contentHeight = useHeight(contentEl);
const actionEl = useTemplateRef('action');
const actionHeight = useHeight(actionEl);

const listHeight = computed(() => contentHeight.value - actionHeight.value);

onBeforeMount(load);

function setCurrentSource(source: KanjiStatsSource) {
  currentSource.value = source;
  if (route.name !== ('snippets' satisfies Route)) {
    go('snippets');
  }
}
</script>

<template>
  <Sidebar>
    <SidebarHeader class="pb-0">
      <div v-if="currentKanji" class="flex flex-col gap-4">
        <div class="flex flex-col items-center justify-center gap-4 pt-2">
          <span class="text-6xl md:text-9xl">{{ currentKanji.character }}</span>
          <Badge>{{ capitalCase(currentKanji.level) }}</Badge>
        </div>
        <div :class="cn('grid gap-4 px-4', currentKanji.quizzes > 0 ? 'grid-cols-4' : 'grid-cols-2')">
          <div class="flex h-16 flex-col items-center justify-center">
            <span class="text-muted-foreground text-xs md:text-sm">Rank</span>
            <span class="text-base md:text-lg font-semibold">{{ ranking ? ranking : '?' }}</span>
          </div>
          <div class="flex h-16 flex-col items-center justify-center">
            <span class="text-muted-foreground text-xs md:text-sm">Total</span>
            <span class="text-base md:text-lg font-semibold">{{ currentKanji.seen }}</span>
          </div>
          <template v-if="currentKanji.quizzes > 0">
            <div class="flex h-16 flex-col items-center justify-center">
              <span class="text-muted-foreground text-xs md:text-sm">Quiz</span>
              <span class="text-base md:text-lg font-semibold">{{ currentKanji.quizzes }}</span>
            </div>
            <div
              v-if="Number.isFinite(currentKanji.quizAccuracy)"
              class="flex h-16 flex-col items-center justify-center"
            >
              <span class="text-muted-foreground text-xs md:text-sm">Accuracy</span>
              <span class="text-base md:text-lg font-semibold">
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
        class="flex size-full flex-col justify-between gap-6 p-2 md:p-4 select-none"
      >
        <ScrollArea :style="{ height: toPixel(listHeight - 40) }">
          <div id="source-grid" class="text-sidebar-accent-foreground pr-4">
            <template v-for="source of currentKanji.sources" :key="source.id">
              <div class="ellipsis cursor-pointer" @click="() => setCurrentSource(source)">
                <span class="text-xs md:text-sm">{{ source.name }}</span>
              </div>
              <div class="text-end">
                <span class="text-xs md:text-sm">{{ source.seen }}</span>
              </div>
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

    <SidebarFooter>
      <DropdownMenu>
        <DropdownMenuTrigger as-child>
          <Button variant="outline">
            <span>{{ capitalCase(String(route.name)) }}</span>
            <ChevronUpIcon class="ml-auto" />
          </Button>
        </DropdownMenuTrigger>

        <DropdownMenuContent side="top" class="w-(--reka-dropdown-menu-trigger-width)">
          <DropdownMenuItem>
            <RouterLink to="/" class="w-full">Home</RouterLink>
          </DropdownMenuItem>
          <DropdownMenuItem>
            <RouterLink to="/snippets" class="w-full">Snippets</RouterLink>
          </DropdownMenuItem>
          <DropdownMenuItem>
            <RouterLink to="/quiz" class="w-full">Quiz</RouterLink>
          </DropdownMenuItem>
          <DropdownMenuItem>
            <RouterLink to="/sources" class="w-full">Sources</RouterLink>
          </DropdownMenuItem>
          <DropdownMenuItem>
            <RouterLink to="/bookmarks" class="w-full">Bookmarks</RouterLink>
          </DropdownMenuItem>
          <DropdownMenuItem>
            <RouterLink to="/settings" class="w-full">Settings</RouterLink>
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </SidebarFooter>
  </Sidebar>
</template>

<style scoped>
#source-grid {
  display: grid;
  grid-template-columns: auto 1fr;
  column-gap: 1rem;
}
</style>
