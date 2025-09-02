<script setup lang="ts">
import { useTemplateRef } from 'vue';
import { toPixel } from '@tb-dev/utils';
import { useKanjis } from '@/composables/kanji';
import { asyncRef, handleError, useHeightDiff } from '@tb-dev/vue';
import { createSource, getSources, renameSource } from '@/commands';
import { Button, Input, Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@tb-dev/vue-components';

const { load: loadKanjis, loading } = useKanjis();
const { state: sources, execute: loadSources } = asyncRef([], getSources);

const topbar = useTemplateRef('topbarEl');
const contentHeight = useHeightDiff(topbar);

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

async function load() {
  await loadSources();
  await loadKanjis();
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
            <TableHead>Path</TableHead>
            <TableHead>Name</TableHead>
          </TableRow>
        </TableHeader>

        <TableBody>
          <TableRow v-for="source of sources" :key="source.id">
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
