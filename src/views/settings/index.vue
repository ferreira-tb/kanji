<script setup lang="ts">
import { Checkbox } from '@tb-dev/vue';
import { useSettingsStore } from '@/stores/settings';

const settings = useSettingsStore();
</script>

<template>
  <div class="flex size-full flex-col gap-6 p-4">
    <h1 class="text-2xl font-semibold">Settings</h1>
    <div class="flex flex-col gap-2">
      <Checkbox v-model="settings.hideOnClose" label="Hide on close" />
      <Checkbox v-model="settings.watchFiles" label="Watch files" />
    </div>
  </div>
</template>
