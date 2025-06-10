<script setup lang="ts">
import { useSettingsStore } from '@/stores/settings';
import { Checkbox, Label } from '@tb-dev/vue-components';

const settings = useSettingsStore();
</script>

<template>
  <div class="flex size-full flex-col gap-6 p-4">
    <h1 class="text-2xl font-semibold">Settings</h1>
    <div class="flex flex-col gap-2">
      <Label>
        <Checkbox v-model="settings.hideOnClose" />
        <span>Hide on close</span>
      </Label>
      <Label>
        <Checkbox v-model="settings.watchFiles" />
        <span>Watch files</span>
      </Label>
    </div>
  </div>
</template>
