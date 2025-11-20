<script setup lang="ts">
import { handleError } from '@/lib/error';
import { useLocalStorage } from '@vueuse/core';
import { BASE_URL_KEY } from '@/lib/local-storage';
import { Input, Label } from '@tb-dev/vue-components';

const baseUrl = useLocalStorage(BASE_URL_KEY, '', {
  initOnMounted: true,
  writeDefaults: true,
  onError: handleError,
});
</script>

<template>
  <div class="flex flex-col gap-4">
    <h2 class="text-xl">General</h2>
    <div class="flex flex-col gap-2">
      <Label class="max-w-70">
        <span>Base URL</span>
        <Input
          v-model="baseUrl"
          type="text"
          spellcheck="false"
          autocapitalize="off"
          autocomplete="off"
        />
      </Label>
    </div>
  </div>
</template>
