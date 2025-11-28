<script setup lang="ts">
import { z } from 'zod';
import { computed, ref } from 'vue';
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
  onStart: (urls: string[]) => Promise<void>;
}>();

const open = defineModel<boolean>('open', { required: true });

const text = ref('');

const schema = z.url({ protocol: /^https$/ });
const urls = computed(() => {
  return text.value
    .split('\n')
    .map((url) => url.trim())
    .filter(validate);
});

async function start() {
  if (urls.value.length) {
    open.value = false;
    await props.onStart(urls.value);
  }
}

function validate(url: string) {
  if (url.length === 0) return false;
  return schema.safeParse(url).success;
}
</script>

<template>
  <Dialog v-model:open="open">
    <DialogTrigger class="hidden"></DialogTrigger>

    <DialogContent class="w-80 md:w-100 max-w-9/10 pb-1">
      <VisuallyHidden>
        <DialogHeader>
          <DialogTitle>Url</DialogTitle>
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
          <Button size="sm" :disabled="disabled || urls.length === 0" @click="start">
            <span>Start</span>
          </Button>
        </div>
      </div>
    </DialogContent>
  </Dialog>
</template>
