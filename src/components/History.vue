<script setup lang="ts">
import { computed, nextTick } from 'vue';
import type { Option } from '@tb-dev/utils';
import { useKanjiStore } from '@/stores/kanji';
import { useKanjis } from '@/composables/kanji';
import { Button, Dialog } from '@tb-dev/vue-components';

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
  <Dialog
    v-model="open"
    header-class="h-[25px] p-6 pb-0"
    content-class="h-[450px] w-[720px] p-0"
    footer-class="h-[25px] p-2"
  >
    <template #trigger>
      <span class="hidden size-0!"></span>
    </template>

    <template #title>
      <span>History</span>
    </template>

    <div class="flex flex-col justify-between gap-4 overflow-x-hidden overflow-y-auto select-none">
      <div class="flex h-[300px] flex-col items-start justify-start gap-2 px-4">
        <template v-for="entry of history" :key="entry">
          <div v-if="entry" class="cursor-pointer" @click="() => set(entry)">
            <span>{{ entry }}</span>
          </div>
        </template>
      </div>
    </div>

    <template #footer>
      <div class="flex max-h-min items-center justify-center gap-2">
        <Button size="sm" :disabled="loading" @click="() => set()">
          <span>Select Folder</span>
        </Button>
        <Button variant="secondary" size="sm" @click="clearHistory">
          <span>Clear History</span>
        </Button>
      </div>
    </template>
  </Dialog>
</template>
