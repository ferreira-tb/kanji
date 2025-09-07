<script setup lang="ts">
import Tauri from './Tauri.vue';
import Website from './Website.vue';
import { asyncRef } from '@tb-dev/vue';
import { getServerAddr } from '@/commands';
import { isTauri } from '@tauri-apps/api/core';

const { state: server } = asyncRef(null, getServerAddr);
</script>

<template>
  <div class="relative flex size-full flex-col gap-6 p-4 overflow-x-hidden overflow-y-auto">
    <div v-if="isTauri() && server" class="absolute right-4 bottom-2">
      <span class="text-muted-foreground text-sm">{{ server }}</span>
    </div>

    <h1 class="text-2xl font-semibold">Settings</h1>

    <Tauri v-if="isTauri()" />
    <Website v-else />
  </div>
</template>
