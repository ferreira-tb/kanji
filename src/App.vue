<script setup lang="ts">
import { onMounted } from 'vue';
import { commands } from '@/api/bindings';
import { useColorMode } from '@vueuse/core';
import { exit } from '@tauri-apps/plugin-process';
import { useSettingsStore } from '@/stores/settings';
import { handleError, onKeyDown } from '@tb-dev/vue';

const settings = useSettingsStore();

useColorMode({
  initialValue: 'dark',
  onError: handleError,
  writeDefaults: true,
});

onKeyDown('Escape', () => exit(0).err());

onMounted(() => {
  // prettier-ignore
  settings.$tauri.start()
    .then(() => commands.showWindow())
    .err()
});
</script>

<template>
  <main class="fixed inset-0 select-none">
    <div class="size-full overflow-hidden p-0">
      <RouterView #default="{ Component }">
        <template v-if="Component">
          <component :is="Component" />
        </template>
      </RouterView>
    </div>
  </main>
</template>
