<script setup lang="ts">
import { computed } from 'vue';
import * as commands from '@/commands';
import { useKanjis } from '@/composables/kanji';
import { useSources } from '@/composables/sources';
import {
  Button,
  Checkbox,
  Input,
  NumberField,
  NumberFieldContent,
  NumberFieldDecrement,
  NumberFieldIncrement,
  NumberFieldInput,
  SidebarTrigger,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@tb-dev/vue-components';

const { loading: isLoadingKanjis, load: loadKanjis } = useKanjis();

const {
  sources,
  loading: isLoadingSources,
  removeSource,
  loadSources,
} = useSources();

const loading = computed(() => {
  return isLoadingKanjis.value || isLoadingSources.value;
});

const desktop = globalThis.__DESKTOP__;

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
    <div class="flex h-14 w-full items-center justify-between gap-4 px-2 md:px-6 py-4">
      <SidebarTrigger />
      <div class="flex items-center justify-center gap-2">
        <Button
          v-if="desktop"
          size="sm"
          :disabled="loading"
          @click="create"
        >
          <span>Add</span>
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
    </div>

    <div class="overflow-x-hidden overflow-y-auto pb-safe-12 px-1 md:px-6">
      <Table v-if="sources.length > 0">
        <TableHeader>
          <TableRow>
            <TableHead></TableHead>
            <TableHead>Path</TableHead>
            <TableHead class="max-md:hidden">Name</TableHead>
            <TableHead class="max-md:hidden">Weight</TableHead>
            <TableHead class="max-lg:hidden"></TableHead>
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
              <span class="break-all wrap-anywhere">{{ source.path }}</span>
            </TableCell>

            <TableCell class="max-md:hidden">
              <div class="flex justify-start items-center gap-2">
                <Input
                  v-model="source.name"
                  type="text"
                  :disabled="loading"
                  @change="() => rename(source)"
                />
              </div>
            </TableCell>

            <TableCell class="max-md:hidden">
              <NumberField
                v-model="source.weight"
                :min="1"
                :max="10"
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

            <TableCell class="max-lg:hidden">
              <div class="flex justify-start items-center gap-2">
                <Button variant="destructive" size="sm" @click="() => removeSource(source.id)">
                  <span>Remove</span>
                </Button>
              </div>
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </div>
  </div>
</template>
