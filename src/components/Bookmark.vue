<script setup lang="ts">
import { StarIcon } from 'lucide-vue-next';
import { Button } from '@tb-dev/vue-components';

defineProps<{
  isBookmarked: boolean;
  disabled?: boolean;
  onClick: () => Promise<void>;
}>();
</script>

<template>
  <Button variant="ghost" :disabled @click="onClick">
    <StarIcon
      :fill="isBookmarked ? 'currentColor' : 'none'"
      class="size-4 lg:size-6 md:size-8"
    />
  </Button>
</template>
