<script setup lang="ts">
import { useHeight } from '@tb-dev/vue';
import { toPixel } from '@tb-dev/utils';
import { useQuiz } from '@/composables/quiz';
import { useSources } from '@/composables/sources';
import { ref, useTemplateRef, watchEffect } from 'vue';
import {
  Button,
  Checkbox,
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

const open = defineModel<boolean>({ required: true });

const { isLoading, ...quiz } = useQuiz();

const { sources } = useSources();
const selectedSources = ref<SourceId[]>([]);

const content = useTemplateRef('contentEl');
const contentHeight = useHeight(content);

watchEffect(() => {
  if (!open.value) {
    selectedSources.value = [];
  }
});

async function start() {
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
    <DialogTrigger class="hidden"></DialogTrigger>

    <DialogContent class="w-80 md:w-100 max-w-9/10 h-120 max-h-3/4 pb-2">
      <VisuallyHidden>
        <DialogHeader>
          <DialogTitle>Select Sources</DialogTitle>
        </DialogHeader>
      </VisuallyHidden>

      <div ref="contentEl" class="size-full px-0 pt-4 overflow-hidden">
        <div class="flex flex-col gap-2 pr-4">
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
              @click="start"
            >
              <span>Start</span>
            </Button>
          </div>
        </div>
      </div>
    </DialogContent>
  </Dialog>
</template>
