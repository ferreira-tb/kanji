<script setup lang="ts">
import type { MaybePromise } from '@tb-dev/utils';
import { Button, DropdownMenuItem, cn } from '@tb-dev/vue-components';
import type { VNode } from 'vue';

defineProps<{
  buttonClass?: string;
  disabled?: boolean;
  onClick: () => MaybePromise<void>;
}>();

defineSlots<{ default: () => VNode; }>();
</script>

<template>
  <DropdownMenuItem as-child>
    <Button
      size="sm"
      variant="ghost"
      :disabled
      :class="cn('w-full justify-start font-normal', buttonClass)"
      @click="() => onClick()"
    >
      <slot></slot>
    </Button>
  </DropdownMenuItem>
</template>
