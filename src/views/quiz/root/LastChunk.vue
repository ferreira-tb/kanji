<script setup lang="ts">
import { since } from '@/lib/date';
import type { LastChunk } from './types';
import { Badge } from '@tb-dev/vue-components';

defineProps<{
  date: LastChunk['date'];
}>();
</script>

<template>
  <Badge variant="outline">
    <span>{{ since(date) }}</span>
  </Badge>
</template>
