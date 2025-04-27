<script setup lang="ts">
import { onMounted } from 'vue';
import { commands } from '@/api/bindings';
import { useColorMode } from '@vueuse/core';
import { exit } from '@tauri-apps/plugin-process';
import { useCounterStore } from '@/stores/counter';
import { handleError, onKeyDown } from '@tb-dev/vue';

const counterStore = useCounterStore();

useColorMode({
  initialValue: 'dark',
  onError: handleError,
  writeDefaults: true,
});

onKeyDown('Escape', () => exit(0).err());

onMounted(() => {
  // prettier-ignore
  counterStore.$tauri.start()
    .then(() => commands.showWindow())
    .err()
});
</script>

<template>
  <main class="h-screen w-screen select-none">
    <div class="size-full overflow-hidden p-0">
      <RouterView #default="{ Component }">
        <template v-if="Component">
          <component :is="Component" />
        </template>
      </RouterView>
    </div>
  </main>
</template>
