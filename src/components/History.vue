<script setup lang="ts">
import { computed, nextTick } from 'vue';
import type { Option } from '@tb-dev/utils';
import { useKanjiStore } from '@/stores/kanji';
import { useKanjis } from '@/composables/kanji';
import {
  Button,
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@tb-dev/vue-components';

const open = defineModel<boolean>({ required: true });

const store = useKanjiStore();
const history = computed(() => {
  return store.history.toReversed();
});

const { loading } = useKanjis();

async function set(entry?: Option<string>) {
  await store.setFolder(entry);
  await nextTick();
  open.value = false;
}

function clearHistory() {
  store.history = [];
}
</script>

<template>
  <Dialog v-model:open="open">
    <DialogTrigger class="hidden size-0!" />

    <DialogContent class="h-[450px] w-[720px] p-0">
      <DialogHeader class="h-[25px] p-6 pb-0">
        <DialogTitle>
          <span>History</span>
        </DialogTitle>
      </DialogHeader>

      <div class="flex flex-col justify-between gap-4 overflow-x-hidden overflow-y-auto select-none">
        <div class="flex h-[300px] flex-col items-start justify-start gap-2 px-4">
          <template v-for="entry of history" :key="entry">
            <div v-if="entry" class="cursor-pointer" @click="() => set(entry)">
              <span>{{ entry }}</span>
            </div>
          </template>
        </div>
      </div>

      <DialogFooter class="h-[25px] p-2">
        <div class="flex max-h-min items-center justify-center gap-2">
          <Button size="sm" :disabled="loading" @click="() => set()">
            <span>Select Folder</span>
          </Button>
          <Button variant="secondary" size="sm" @click="clearHistory">
            <span>Clear History</span>
          </Button>
        </div>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
