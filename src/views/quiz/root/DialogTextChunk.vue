<script setup lang="ts">
import { ref, watchEffect } from 'vue';
import { trimArray } from '@tb-dev/utils';
import {
  Button,
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
  Textarea,
  VisuallyHidden,
} from '@tb-dev/vue-components';

const props = defineProps<{
  disabled: boolean;
  onStart: (chars: readonly string[]) => Promise<void>;
}>();

const open = defineModel<boolean>('open', { required: true });

const text = ref('');

watchEffect(() => {
  if (!open.value) {
    text.value = '';
  }
});

async function start() {
  let chars = Array.from(text.value.trim());
  chars = trimArray(chars, { allowEmpty: false });
  if (chars.length > 0) {
    open.value = false;
    await props.onStart(chars);
  }
}
</script>

<template>
  <Dialog v-model:open="open">
    <DialogTrigger class="hidden"></DialogTrigger>

    <DialogContent class="w-80 md:w-100 max-w-9/10 pb-2">
      <VisuallyHidden>
        <DialogHeader>
          <DialogTitle>Custom Text Chunk</DialogTitle>
        </DialogHeader>
      </VisuallyHidden>

      <div class="flex flex-col gap-4 px-2 py-8">
        <Textarea
          v-model="text"
          :disabled
          autocomplete="off"
          autocorrect="off"
          spellcheck="false"
          class="min-h-20"
        />

        <div class="flex justify-center items-center">
          <Button size="sm" :disabled="disabled || text.length === 0" @click="start">
            <span>Start</span>
          </Button>
        </div>
      </div>
    </DialogContent>
  </Dialog>
</template>
