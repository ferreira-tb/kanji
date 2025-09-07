<script setup lang="ts">
import Tauri from './Tauri.vue';
import Website from './Website.vue';
import { useTemplateRef } from 'vue';
import { toPixel } from '@tb-dev/utils';
import { getServerAddr } from '@/commands';
import { isTauri } from '@tauri-apps/api/core';
import { asyncRef, useHeightDiff } from '@tb-dev/vue';
import { SidebarTrigger } from '@tb-dev/vue-components';

const { state: server } = asyncRef(null, getServerAddr);

const topbar = useTemplateRef('topbarEl');
const contentHeight = useHeightDiff(topbar);
</script>

<template>
  <div class="flex size-full flex-col gap-2">
    <div ref="topbarEl" class="flex h-14 w-full items-center justify-between px-2 md:px-6 py-4">
      <SidebarTrigger />
      <div v-if="isTauri() && server">
        <span class="text-muted-foreground text-sm">{{ server }}</span>
      </div>
    </div>

    <div
      class="flex flex-col gap-2 md:gap-4 overflow-x-hidden overflow-y-auto px-1 md:px-6 pb-12"
      :style="{ height: toPixel(contentHeight) }"
    >
      <Tauri v-if="isTauri()" />
      <Website v-else />
    </div>
  </div>
</template>
