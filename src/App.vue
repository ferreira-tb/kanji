<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { onMounted, ref } from 'vue';
import { useRoute } from 'vue-router';
import { commands } from '@/api/bindings';
import { capitalCase } from 'change-case';
import { useColorMode } from '@vueuse/core';
import { ChevronUp } from 'lucide-vue-next';
import { useKanjiStore } from '@/stores/kanji';
import { exit } from '@tauri-apps/plugin-process';
import { useRanking } from '@/composables/ranking';
import { handleError, onKeyDown } from '@tb-dev/vue';
import {
  Badge,
  Button,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
  Sidebar,
} from '@tb-dev/vue-components';

const store = useKanjiStore();
const { selected } = storeToRefs(store);
const ranking = useRanking(selected);

const route = useRoute();
const isSidebarOpen = ref(true);

useColorMode({
  initialValue: 'dark',
  onError: handleError,
  writeDefaults: true,
});

onKeyDown('Escape', () => exit(0).err());

onMounted(async () => {
  try {
    await commands.createTrayIcon();
    await commands.showWindow();
  } catch (err) {
    handleError(err);
  }
});
</script>

<template>
  <Sidebar v-model:open="isSidebarOpen" width="250px">
    <main class="h-screen w-[calc(100vw-var(--sidebar-width))] select-none">
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

    <template #content>
      <div v-if="selected" class="flex h-full flex-col gap-4 p-4 select-none">
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
    </template>

    <template #footer>
      <DropdownMenu>
        <DropdownMenuTrigger as-child>
          <Button v-if="typeof route.name === 'string'" variant="outline">
            <span>{{ capitalCase(route.name) }}</span>
            <ChevronUp class="ml-auto" />
          </Button>
        </DropdownMenuTrigger>

        <DropdownMenuContent side="top" class="w-[var(--reka-dropdown-menu-trigger-width)]">
          <DropdownMenuItem>
            <RouterLink to="/" class="w-full">Home</RouterLink>
          </DropdownMenuItem>
          <DropdownMenuItem>
            <RouterLink to="/settings" class="w-full">Settings</RouterLink>
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </template>
  </Sidebar>
</template>

<style scoped>
#source-grid {
  display: grid;
  grid-template-columns: auto 1fr;
  column-gap: 1rem;
}
</style>
