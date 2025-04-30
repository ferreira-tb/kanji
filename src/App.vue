<script setup lang="ts">
import { onMounted } from 'vue';
import { storeToRefs } from 'pinia';
import { capitalCase } from 'change-case';
import { commands } from '@/api/bindings';
import { useColorMode } from '@vueuse/core';
import { exit } from '@tauri-apps/plugin-process';
import { useRanking } from '@/composables/ranking';
import { useSettingsStore } from '@/stores/settings';
import { Badge, handleError, onKeyDown, Sidebar } from '@tb-dev/vue';

const settings = useSettingsStore();
const { selected } = storeToRefs(settings);

const ranking = useRanking(selected);

useColorMode({
  initialValue: 'dark',
  onError: handleError,
  writeDefaults: true,
});

onKeyDown('Escape', () => exit(0).err());

onMounted(() => {
  // prettier-ignore
  settings.$tauri.start()
    .then(() => commands.createTrayIcon())
    .then(() => commands.showWindow())
    .err()
});
</script>

<template>
  <Sidebar default-open width="250px">
    <main class="h-screen w-[calc(100vw-var(--sidebar-width))] select-none">
      <div class="size-full overflow-hidden p-0">
        <RouterView #default="{ Component }">
          <template v-if="Component">
            <component :is="Component" />
          </template>
        </RouterView>
      </div>
    </main>

    <template #content>
      <div v-if="selected" class="flex flex-col gap-4 p-4 select-none">
        <div class="flex flex-col items-center justify-center gap-4 pt-4">
          <span class="text-9xl">{{ selected.kanji.character }}</span>
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
  </Sidebar>
</template>

<style scoped>
#source-grid {
  display: grid;
  grid-template-columns: auto 1fr;
  column-gap: 1rem;
}
</style>
