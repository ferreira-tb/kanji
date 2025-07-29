<script setup lang="ts">
import { go } from '@/router';
import { onMounted } from 'vue';
import { storeToRefs } from 'pinia';
import { useRoute } from 'vue-router';
import { capitalCase } from 'change-case';
import { useKanjiStore } from '@/stores/kanji';
import History from '@/components/History.vue';
import { ChevronUpIcon } from 'lucide-vue-next';
import { useKanjis } from '@/composables/kanji';
import { exit } from '@tauri-apps/plugin-process';
import { useRanking } from '@/composables/ranking';
import { useColorMode, useToggle } from '@vueuse/core';
import { createTrayIcon, showWindow } from '@/commands';
import { handleError, onCtrlKeyDown, onKeyDown } from '@tb-dev/vue';
import {
  Badge,
  Button,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarProvider,
} from '@tb-dev/vue-components';

const store = useKanjiStore();
const { selected } = storeToRefs(store);
const ranking = useRanking(selected);

const { kanjis, loading, next, previous } = useKanjis();

const route = useRoute();
const [isSidebarOpen] = useToggle(true);
const [isHistoryOpen, toggleHistory] = useToggle(false);

useColorMode({
  initialValue: 'dark',
  onError: handleError,
  writeDefaults: true,
});

onKeyDown('F1', () => go('home'));
onKeyDown('F2', () => go('snippets'));
onKeyDown('F3', () => go('settings'));
onKeyDown('Escape', () => exit(0).err());

onCtrlKeyDown(['o', 'O'], () => toggleHistory());

onMounted(async () => {
  try {
    await createTrayIcon();
    await showWindow();
  }
  catch (err) {
    handleError(err);
  }
});
</script>

<template>
  <SidebarProvider v-model:open="isSidebarOpen" style="--sidebar-width: 20rem">
    <Sidebar>
      <SidebarContent>
        <div v-if="selected" class="flex h-full flex-col justify-between gap-6 p-4 select-none">
          <div class="flex flex-col gap-4">
            <div class="flex flex-col items-center justify-center gap-4 pt-4">
              <span class="text-9xl">{{ selected.character }}</span>
              <Badge>{{ capitalCase(selected.level) }}</Badge>
            </div>
            <div class="grid grid-cols-2 gap-4 px-4">
              <div class="flex h-16 flex-col items-center justify-center">
                <span class="text-muted-foreground text-sm">Rank</span>
                <span class="text-lg font-semibold">{{ ranking ? ranking : '?' }}</span>
              </div>
              <div class="flex h-16 flex-col items-center justify-center">
                <span class="text-muted-foreground text-sm">Total</span>
                <span class="text-lg font-semibold">{{ selected.seen }}</span>
              </div>
            </div>
            <div id="source-grid" class="text-sidebar-accent-foreground text-sm">
              <template v-for="source of selected.sources" :key="source.name">
                <div>{{ source.name }}</div>
                <div class="text-end">{{ source.seen }}</div>
              </template>
            </div>
          </div>

          <div class="grid grid-cols-2 items-center gap-4">
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
            <Button v-if="typeof route.name === 'string'" variant="outline">
              <span>{{ capitalCase(route.name) }}</span>
              <ChevronUpIcon class="ml-auto" />
            </Button>
          </DropdownMenuTrigger>

          <DropdownMenuContent side="top" class="w-[var(--reka-dropdown-menu-trigger-width)]">
            <DropdownMenuItem>
              <RouterLink to="/" class="w-full">Home</RouterLink>
            </DropdownMenuItem>
            <DropdownMenuItem>
              <RouterLink to="/snippets" class="w-full">Snippets</RouterLink>
            </DropdownMenuItem>
            <DropdownMenuItem>
              <RouterLink to="/settings" class="w-full">Settings</RouterLink>
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </SidebarFooter>
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

    <History v-model="isHistoryOpen" />
  </SidebarProvider>
</template>

<style scoped>
#source-grid {
  display: grid;
  grid-template-columns: auto 1fr;
  column-gap: 1rem;
}
</style>
