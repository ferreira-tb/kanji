<script setup lang="ts">
import { ref, watchEffect } from 'vue';
import * as commands from '@/commands';
import { handleError } from '@/lib/error';
import DialogCreate from './DialogCreate.vue';
import DialogSources from '@/components/DialogSources.vue';
import { useSourceGroups } from '@/composables/source-groups';
import {
  Button,
  Input,
  SidebarTrigger,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@tb-dev/vue-components';

const {
  sourceGroups,
  loading,
  loadSourceGroups,
  removeSourceGroup,
} = useSourceGroups();

const isDialogCreateOpen = ref(false);
const isDialogSourcesOpen = ref(false);

const currentSourceGroudId = ref<Option<SourceGroupId>>(null);
const selectedSources = ref<SourceId[]>([]);

watchEffect(() => {
  if (!isDialogSourcesOpen.value) {
    currentSourceGroudId.value = null;
  }
});

async function openSourcesDialog(id: SourceGroupId) {
  try {
    selectedSources.value = await commands.getSourceGroupSourceIds(id);
    currentSourceGroudId.value = id;
    isDialogSourcesOpen.value = true;
  }
  catch (err) {
    selectedSources.value = [];
    handleError(err);
  }
}

function rename(group: SourceGroup) {
  commands.renameSourceGroup(group.id, group.name)
    .then(loadSourceGroups)
    .err();
}

function setSources(sources: readonly SourceId[]) {
  const id = currentSourceGroudId.value;
  if (id) {
    isDialogSourcesOpen.value = false;
    commands.setSourceGroupSources(id, sources)
      .then(loadSourceGroups)
      .err();
  }
}
</script>

<template>
  <div class="flex size-full flex-col gap-2">
    <div class="flex h-14 w-full items-center justify-between gap-4 px-2 md:px-6 py-4">
      <SidebarTrigger />

      <DialogCreate
        v-model="isDialogCreateOpen"
        :disabled="loading"
        @load="loadSourceGroups"
      />

      <DialogSources
        v-model:open="isDialogSourcesOpen"
        v-model:selected="selectedSources"
        confirm-label="Save"
        :disabled="loading"
        :disable-if-empty="false"
        @confirm="(sources) => setSources(sources)"
      />

      <div class="flex items-center justify-center gap-2">
        <Button
          size="sm"
          variant="default"
          :disabled="loading"
          @click="() => void (isDialogCreateOpen = true)"
        >
          <span>Create</span>
        </Button>

        <Button
          size="sm"
          variant="secondary"
          :disabled="loading"
          @click="loadSourceGroups"
        >
          <span>Reload</span>
        </Button>
      </div>
    </div>

    <div class="overflow-x-hidden overflow-y-auto pb-safe-12 px-1 md:px-6">
      <Table v-if="sourceGroups.length > 0">
        <TableHeader>
          <TableRow class="bg-background hover:bg-background">
            <TableHead>Name</TableHead>
            <TableHead></TableHead>
          </TableRow>
        </TableHeader>

        <TableBody>
          <TableRow v-for="group of sourceGroups" :key="group.id">
            <TableCell>
              <div class="flex justify-start items-center gap-2">
                <Input
                  v-model="group.name"
                  type="text"
                  :disabled="loading"
                  @change="() => rename(group)"
                />
              </div>
            </TableCell>

            <TableCell>
              <div class="flex justify-start items-center gap-2">
                <Button
                  size="sm"
                  variant="default"
                  :disabled="loading"
                  @click="() => openSourcesDialog(group.id)"
                >
                  <span>Sources</span>
                </Button>

                <Button
                  variant="destructive"
                  size="sm"
                  @click="() => removeSourceGroup(group.id)"
                >
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
