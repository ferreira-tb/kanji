<script setup lang="ts">
import { useHeight } from '@tb-dev/vue';
import { useQuiz } from '@/composables/quiz';
import { useSources } from '@/composables/sources';
import { toPixel, unreachable } from '@tb-dev/utils';
import { computed, nextTick, ref, useTemplateRef, type VNode, watchEffect } from 'vue';
import {
  Button,
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
  ScrollArea,
  VisuallyHidden,
} from '@tb-dev/vue-components';

defineSlots<{
  trigger: () => VNode;
}>();

const { isLoading, ...quiz } = useQuiz();
const { sources } = useSources();

const enum State {
  Root = 0,
  SourceList = 1,
}

const state = ref<State>(State.Root);

const open = ref(false);
const content = useTemplateRef('contentEl');
const contentHeight = useHeight(content);
const contentClass = computed(() => {
  let classList = 'w-80';
  switch (state.value) {
    case State.Root: {
      classList += ' h-60';
      break;
    }
    case State.SourceList: {
      classList += ' h-96 max-h-3/4';
      break;
    }
  }

  return classList;
});

watchEffect(() => {
  if (!open.value) {
    void nextTick(() => state.value = State.Root);
  }
});

async function start(quizKind: QuizKind['kind']) {
  switch (quizKind) {
    case 'random-chunk': {
      open.value = false;
      await quiz.startRandomChunk();
      break;
    }
    case 'random-source': {
      open.value = false;
      await quiz.startRandomSource();
      break;
    }
    case 'source': {
      state.value = State.SourceList;
      break;
    }
    default: {
      unreachable();
    }
  }
}

async function startSource(id: SourceId) {
  open.value = false;
  await quiz.startSource(id);
}
</script>

<template>
  <Dialog v-model:open="open">
    <DialogTrigger as-child>
      <slot name="trigger"></slot>
    </DialogTrigger>
    <DialogContent :class="contentClass">
      <VisuallyHidden>
        <DialogHeader>
          <DialogTitle>Start new quiz</DialogTitle>
        </DialogHeader>
      </VisuallyHidden>

      <div v-if="open" ref="contentEl" class="size-full px-0 py-4 overflow-hidden">
        <div v-if="state === State.Root" class="h-full flex justify-center items-center">
          <div class="h-max grid grid-cols-1 gap-2 md:gap-4">
            <Button
              size="sm"
              :disabled="isLoading"
              @click="() => start('random-chunk')"
            >
              <span>Random Chunk</span>
            </Button>

            <Button
              size="sm"
              :disabled="isLoading || sources.length === 0"
              @click="() => start('source')"
            >
              <span>Source</span>
            </Button>

            <Button
              size="sm"
              :disabled="isLoading || sources.length === 0"
              @click="() => start('random-source')"
            >
              <span>Random Source</span>
            </Button>
          </div>
        </div>

        <ScrollArea
          v-else-if="state === State.SourceList"
          :style="{ height: toPixel(contentHeight - 20) }"
        >
          <div class="flex flex-col gap-1 pr-4">
            <Button
              v-for="source of sources"
              :key="source.id"
              variant="ghost"
              size="sm"
              :disabled="isLoading"
              class="justify-start"
              @click="() => startSource(source.id)"
            >
              <span>{{ source.name }}</span>
            </Button>
          </div>
        </ScrollArea>
      </div>
    </DialogContent>
  </Dialog>
</template>
