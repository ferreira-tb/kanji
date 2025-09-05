<script setup lang="ts">
import { toPixel } from '@tb-dev/utils';
import { computed, useTemplateRef } from 'vue';
import { useKanjis } from '@/composables/kanji';
import { asyncRef, handleError, useHeightDiff } from '@tb-dev/vue';
import { createSource, getSources, renameSource, toggleSource } from '@/commands';
import {
  Button,
  Checkbox,
  Input,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
  toBooleanCheckboxValue,
} from '@tb-dev/vue-components';

const { load: loadKanjis, loading: isLoadingKanjis } = useKanjis();

const {
  state: sources,
  execute: loadSources,
  isLoading: isLoadingSources,
} = asyncRef([], getSources);

const loading = computed(() => {
  return isLoadingKanjis.value || isLoadingSources.value;
});

const topbar = useTemplateRef('topbarEl');
const contentHeight = useHeightDiff(topbar);

async function load() {
  await loadSources();
  await loadKanjis();
}

async function addSource() {
  try {
    await createSource();
    await load();
  }
  catch (err) {
    handleError(err);
  }
}

async function rename(source: Source) {
  try {
    await renameSource(source.id, source.name);
    await load();
  }
  catch (err) {
    handleError(err);
  }
}

async function toggle(source: Source, enabled: boolean) {
  try {
    await toggleSource(source.id, enabled);
    await load();
  }
  catch (err) {
    handleError(err);
  }
}
</script>

<template>
  <div class="flex size-full flex-col gap-2">
    <div ref="topbarEl" class="flex h-14 w-full items-center justify-end gap-4 px-6 py-4">
      <Button
        size="sm"
        :disabled="loading"
        @click="addSource"
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
          </TableRow>
        </TableHeader>

        <TableBody>
          <TableRow v-for="source of sources" :key="source.id">
            <TableCell>
              <Checkbox
                v-model="source.enabled"
                :disabled="loading"
                @update:model-value="(value) => toggle(source, toBooleanCheckboxValue(value))"
              />
            </TableCell>
            <TableCell>
              <span>{{ source.path }}</span>
            </TableCell>
            <TableCell>
              <div class="flex justify-start items-center gap-2">
                <Input v-model="source.name" type="text" />
                <Button
                  size="sm"
                  variant="secondary"
                  :disabled="loading"
                  @click="() => rename(source)"
                >
                  <span>Rename</span>
                </Button>
              </div>
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </div>
  </div>
</template>
