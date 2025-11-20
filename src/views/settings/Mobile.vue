<script setup lang="ts">
import { useSettingsStore } from '@/stores/settings';
import { Input, Label } from '@tb-dev/vue-components';

const settings = useSettingsStore();
</script>

<template>
  <div class="flex flex-col gap-4">
    <h2 class="text-xl">General</h2>
    <div class="flex flex-col gap-2">
      <Label class="max-w-70">
        <span>Base URL</span>
        <Input
          v-model="settings.baseUrl"
          type="text"
          spellcheck="false"
          autocapitalize="off"
          autocomplete="off"
        />
      </Label>
    </div>
  </div>
</template>
