<script setup lang="ts">
import { useHeight } from '@tb-dev/vue';
import { useQuiz } from '@/composables/quiz';
import { useSources } from '@/composables/sources';
import { toPixel, unreachable } from '@tb-dev/utils';
import { computed, nextTick, ref, useTemplateRef, type VNode, watchEffect } from 'vue';
import {
  Button,
  Checkbox,
  cn,
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
  ScrollArea,
  Table,
  TableBody,
  TableCell,
  TableRow,
  toBooleanCheckboxValue,
  VisuallyHidden,
} from '@tb-dev/vue-components';

defineSlots<{
  trigger: () => VNode;
}>();

const { isLoading, ...quiz } = useQuiz();

const { sources } = useSources();
const selectedSources = ref<SourceId[]>([]);

const enum State {
  Root = 0,
  SourceList = 1,
}

const state = ref<State>(State.Root);

const open = ref(false);
const content = useTemplateRef('contentEl');
const contentHeight = useHeight(content);
const contentClass = computed(() => {
  let classList = 'w-80 max-w-9/10 pb-2 ';
  switch (state.value) {
    case State.Root: {
      classList += 'h-60';
      break;
    }
    case State.SourceList: {
      classList += 'md:w-100 h-120 max-h-3/4';
      break;
    }
  }

  return cn(classList);
});

watchEffect(() => {
  if (!open.value) {
    void nextTick(() => {
      state.value = State.Root;
      selectedSources.value = [];
    });
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

async function startSource() {
  open.value = false;
  if (selectedSources.value.length > 0) {
    await quiz.startSource(selectedSources.value);
  }
}

function onSourceChecked(id: SourceId, checked: boolean | 'indeterminate') {
  checked = toBooleanCheckboxValue(checked);
  if (checked && !selectedSources.value.includes(id)) {
    selectedSources.value.push(id);
  }
  else if (!checked) {
    selectedSources.value = selectedSources.value.filter((source) => {
      return source !== id;
    });
  }
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

      <div v-if="open" ref="contentEl" class="size-full px-0 pt-4 overflow-hidden">
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

        <div v-else-if="state === State.SourceList" class="flex flex-col gap-2 pr-4">
          <ScrollArea :style="{ height: toPixel(contentHeight - 60) }">
            <Table v-if="sources.length > 0">
              <TableBody>
                <TableRow v-for="source of sources" :key="source.id">
                  <TableCell>
                    <Checkbox
                      :model-value="selectedSources.includes(source.id)"
                      :disabled="isLoading"
                      class="disabled:cursor-default"
                      @update:model-value="(checked) => onSourceChecked(source.id, checked)"
                    />
                  </TableCell>

                  <TableCell
                    class="cursor-pointer select-none"
                    @click="() => onSourceChecked(source.id, !selectedSources.includes(source.id))"
                  >
                    <span class="break-all wrap-anywhere">{{ source.name }}</span>
                  </TableCell>
                </TableRow>
              </TableBody>
            </Table>
          </ScrollArea>

          <div class="flex justify-center items-center">
            <Button
              size="sm"
              :disabled="isLoading || selectedSources.length === 0"
              @click="startSource"
            >
              <span>Start</span>
            </Button>
          </div>
        </div>
      </div>
    </DialogContent>
  </Dialog>
</template>
