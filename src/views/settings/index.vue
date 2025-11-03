<script setup lang="ts">
import Mobile from './Mobile.vue';
import Desktop from './Desktop.vue';
import { asyncRef } from '@tb-dev/vue';
import { getServerAddr } from '@/commands';
import { checkForUpdates } from '@/lib/updater';
import { Button, SidebarTrigger } from '@tb-dev/vue-components';

const { state: server } = asyncRef(null, getServerAddr);

const desktop = globalThis.__DESKTOP__;
const version = globalThis.__VERSION__;
</script>

<template>
  <div class="flex size-full flex-col gap-2">
    <div class="flex h-14 w-full items-center justify-between px-2 md:px-6 py-4">
      <SidebarTrigger />

      <div class="flex items-center justify-center gap-4">
        <template v-if="desktop && server">
          <span class="text-muted-foreground text-sm">{{ server }}</span>
          <div class="flex items-center justify-center gap-2">
            <Button size="sm" @click="checkForUpdates">
              <span>Check for updates</span>
            </Button>
          </div>
        </template>
        <span v-else class="text-muted-foreground text-sm">{{ `v${version}` }}</span>
      </div>
    </div>

    <div class="flex flex-col gap-6 md:gap-4 overflow-x-hidden overflow-y-auto px-4 md:px-6 pb-safe-12">
      <Desktop v-if="desktop" />
      <Mobile v-else />
    </div>
  </div>
</template>
