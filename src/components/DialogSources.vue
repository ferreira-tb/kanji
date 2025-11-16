<script setup lang="ts">
import { useHeight } from '@tb-dev/vue';
import { useSources } from '@/composables/sources';
import { type MaybePromise, toPixel } from '@tb-dev/utils';
import { computed, useTemplateRef, watchEffect } from 'vue';
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

const props = defineProps<{
  confirmLabel: string;
  disabled: boolean;
  disableIfEmpty: boolean;
  onConfirm: (sourceIds: readonly SourceId[]) => MaybePromise<void>;
}>();

const open = defineModel<boolean>('open', { required: true });
const selected = defineModel<SourceId[]>('selected', { required: true });

const { sources, loading } = useSources();
const shouldDisable = computed(() => props.disabled || loading.value);

const content = useTemplateRef('contentEl');
const contentHeight = useHeight(content);

watchEffect(() => {
  if (!open.value) {
    selected.value = [];
  }
});

async function confirm() {
  open.value = false;
  await props.onConfirm(selected.value);
}

function onSourceChecked(id: SourceId, checked: boolean | 'indeterminate') {
  checked = toBooleanCheckboxValue(checked);
  if (checked && !selected.value.includes(id)) {
    selected.value.push(id);
  }
  else if (!checked) {
    selected.value = selected.value.filter((source) => {
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
                  <TableCell class="w-max pr-2!">
                    <Checkbox
                      :model-value="selected.includes(source.id)"
                      :disabled="shouldDisable"
                      class="disabled:cursor-default"
                      @update:model-value="(checked) => onSourceChecked(source.id, checked)"
                    />
                  </TableCell>

                  <TableCell
                    class="w-full cursor-pointer select-none"
                    @click="() => onSourceChecked(source.id, !selected.includes(source.id))"
                  >
                    <span class="break-all wrap-anywhere">{{ source.name }}</span>
                  </TableCell>
                </TableRow>
              </TableBody>
            </Table>
          </ScrollArea>

          <div class="flex justify-center items-center gap-2">
            <Button
              size="sm"
              variant="default"
              :disabled="shouldDisable || (disableIfEmpty && selected.length === 0)"
              @click="confirm"
            >
              <span>{{ confirmLabel }}</span>
            </Button>
            <Button
              size="sm"
              variant="secondary"
              :disabled="shouldDisable"
              @click="() => void (selected = [])"
            >
              <span>Clear</span>
            </Button>
          </div>
        </div>
      </div>
    </DialogContent>
  </Dialog>
</template>
