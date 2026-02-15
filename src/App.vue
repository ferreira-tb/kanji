<script setup lang="ts">
import { go } from '@/router';
import { onMounted, ref } from 'vue';
import * as commands from '@/commands';
import { useColorMode } from '@vueuse/core';
import Sidebar from '@/components/Sidebar.vue';
import { exit } from '@tauri-apps/plugin-process';
import { SidebarProvider, Sonner } from '@tb-dev/vue-components';
import { handleError, onKeyDown, useBreakpoints } from '@tb-dev/vue';

const isSidebarOpen = ref(true);

const { md } = useBreakpoints();

useColorMode({
  initialValue: 'dark',
  onError: handleError,
  writeDefaults: true,
});

if (__DESKTOP__) {
  onKeyDown('F1', () => go('home'));
  onKeyDown('F2', () => go('snippets'));
  onKeyDown('F3', () => go('quiz'));
  onKeyDown('F4', () => go('sources'));
  onKeyDown('F6', () => go('bookmarks'));
  onKeyDown('F7', () => go('settings'));
  onKeyDown('Escape', () => exit(0).err());
}

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
    <Sonner :position="md ? 'bottom-right' : 'top-center'" />

    <main class="h-screen w-full select-none">
      <div class="size-full overflow-hidden p-0">
        <RouterView #default="{ Component }">
          <template v-if="Component">
            <component :is="Component" />
          </template>
        </RouterView>
      </div>
    </main>
  </SidebarProvider>
</template>
