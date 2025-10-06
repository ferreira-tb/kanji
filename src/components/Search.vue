<script setup lang="ts">
import { SearchIcon } from 'lucide-vue-next';
import { Input } from '@tb-dev/vue-components';

const search = defineModel<Option<string>>({ required: true });
</script>

<template>
  <div class="w-48 lg:w-60 xl:w-96">
    <div class="relative flex w-full items-center">
      <Input v-model.trim="search" type="text" placeholder="Search" class="w-full pl-10" />
      <span class="absolute inset-y-0 start-0 flex items-center justify-center px-2">
        <SearchIcon class="text-muted-foreground size-6" />
      </span>
    </div>
  </div>
</template>
