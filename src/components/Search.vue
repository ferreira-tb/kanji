<script setup lang="ts">
import type { Option } from '@tb-dev/utils';
import { InputText } from '@tb-dev/vue-components';
import { Search as SearchIcon } from 'lucide-vue-next';

const search = defineModel<Option<string>>({ required: true });
</script>

<template>
  <div>
    <div class="relative flex w-full items-center">
      <InputText v-model="search" placeholder="Search" class="w-full pl-10" />
      <span class="absolute inset-y-0 start-0 flex items-center justify-center px-2">
        <SearchIcon class="text-muted-foreground size-6" />
      </span>
    </div>
  </div>
</template>
