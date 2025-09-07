<script setup lang="ts">
import * as commands from '@/commands';
import { toPixel } from '@tb-dev/utils';
import { useHeightDiff } from '@tb-dev/vue';
import { computed, useTemplateRef } from 'vue';
import { useKanjis } from '@/composables/kanji';
import { useSources } from '@/composables/source';
import {
  Button,
  Checkbox,
  Input,
  NumberField,
  NumberFieldContent,
  NumberFieldDecrement,
  NumberFieldIncrement,
  NumberFieldInput,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@tb-dev/vue-components';

const { load: loadKanjis, loading: isLoadingKanjis } = useKanjis();
const { sources, loadSources, loading: isLoadingSources } = useSources();

const loading = computed(() => {
  return isLoadingKanjis.value || isLoadingSources.value;
});

const topbar = useTemplateRef('topbarEl');
const contentHeight = useHeightDiff(topbar);

async function load() {
  await loadSources();
  await loadKanjis();
}

function create() {
  commands.createSource().then(load).err();
}

function rename(source: Source) {
  commands.renameSource(source.id, source.name).then(load).err();
}

function setWeight(source: Source) {
  commands.setSourceWeight(source.id, source.weight).then(load).err();
}

function toggle(source: Source) {
  commands.toggleSource(source.id, source.enabled).then(load).err();
}
</script>

<template>
  <div class="flex size-full flex-col gap-2">
    <div ref="topbarEl" class="flex h-14 w-full items-center justify-end gap-4 px-6 py-4">
      <Button
        size="sm"
        :disabled="loading"
        @click="create"
      >
        <span>Add Source</span>
      </Button>
      <Button
        size="sm"
        variant="secondary"
        :disabled="loading"
        @click="load"
      >
        <span>Reload</span>
      </Button>
    </div>

    <div
      class="overflow-x-hidden overflow-y-auto pb-12 px-6"
      :style="{ height: toPixel(contentHeight) }"
    >
      <Table v-if="sources.length > 0">
        <TableHeader>
          <TableRow>
            <TableHead></TableHead>
            <TableHead>Path</TableHead>
            <TableHead>Name</TableHead>
            <TableHead>Weight</TableHead>
          </TableRow>
        </TableHeader>

        <TableBody>
          <TableRow v-for="source of sources" :key="source.id">
            <TableCell>
              <Checkbox
                v-model="source.enabled"
                :disabled="loading"
                @update:model-value="() => toggle(source)"
              />
            </TableCell>

            <TableCell>
              <span>{{ source.path }}</span>
            </TableCell>

            <TableCell>
              <div class="flex justify-start items-center gap-2">
                <Input
                  v-model="source.name"
                  type="text"
                  :disabled="loading"
                  @change="() => rename(source)"
                />
              </div>
            </TableCell>

            <TableCell>
              <NumberField
                v-model="source.weight"
                :min="1"
                :max="5"
                :step="1"
                :disabled="loading"
                class="max-w-28"
                @update:model-value="() => setWeight(source)"
              >
                <NumberFieldContent>
                  <NumberFieldDecrement />
                  <NumberFieldInput class="dark:bg-input/40" />
                  <NumberFieldIncrement />
                </NumberFieldContent>
              </NumberField>
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </div>
  </div>
</template>
