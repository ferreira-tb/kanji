<script setup lang="ts">
import { ref, watchEffect } from 'vue';
import * as commands from '@/commands';
import {
  Button,
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
  Input,
  Label,
  VisuallyHidden,
} from '@tb-dev/vue-components';

const props = defineProps<{
  disabled: boolean;
  onLoad: () => Promise<void>;
}>();

const open = defineModel<boolean>({ required: true });

const name = ref('');

watchEffect(() => {
  if (!open.value) {
    name.value = '';
  }
});

async function create() {
  const group = name.value.trim();
  if (group.length > 0) {
    await commands.createSourceGroup(group);
    open.value = false;
    await props.onLoad();
  }
}
</script>

<template>
  <Dialog v-model:open="open">
    <DialogTrigger class="hidden"></DialogTrigger>

    <DialogContent class="w-80 md:w-100 max-w-9/10 pb-2">
      <VisuallyHidden>
        <DialogHeader>
          <DialogTitle>Create Source Group</DialogTitle>
        </DialogHeader>
      </VisuallyHidden>

      <div class="flex flex-col gap-4 px-2 py-8">
        <Label>
          <span>Name</span>
          <Input v-model="name" type="text" :disabled spellcheck="false" />
        </Label>

        <div class="flex justify-center items-center">
          <Button size="sm" :disabled @click="create">
            <span>Create</span>
          </Button>
        </div>
      </div>
    </DialogContent>
  </Dialog>
</template>
