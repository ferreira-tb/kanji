<script setup lang="ts">
import { go } from '@/router';
import { onMounted } from 'vue';
import * as commands from '@/commands';
import { isTauri } from '@tauri-apps/api/core';
import Sidebar from '@/components/Sidebar.vue';
import { exit } from '@tauri-apps/plugin-process';
import { handleError, onKeyDown } from '@tb-dev/vue';
import { useColorMode, useToggle } from '@vueuse/core';
import { SidebarProvider } from '@tb-dev/vue-components';

const [isSidebarOpen] = useToggle(true);

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
onKeyDown('Escape', () => exit(0).err(), { enabled: isTauri() });

onMounted(async () => {
  try {
    await commands.createTrayIcon();
    await commands.showWindow();
  }
  catch (err) {
    handleError(err);
  }
});
</script>

<template>
  <SidebarProvider v-model:open="isSidebarOpen" style="--sidebar-width: 20rem">
    <Sidebar />
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
