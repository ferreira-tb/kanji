<script setup lang="ts">
import type { VNode } from 'vue';
import type { MaybePromise } from '@tb-dev/utils';
import { Button, cn, DropdownMenuItem } from '@tb-dev/vue-components';

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
