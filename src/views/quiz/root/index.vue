<script setup lang="ts">
import { ref } from 'vue';
import Chunks from './Chunks.vue';
import Question from './Question.vue';
import { useQuiz } from '@/composables/quiz';
import DialogTextChunk from './DialogTextChunk.vue';
import DialogSources from '@/components/DialogSources.vue';
import DialogSourceGroups from '@/components/DialogSourceGroups.vue';
import DropdownMenuButton from '@/components/DropdownMenuButton.vue';
import {
  Button,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuTrigger,
  Loading,
  SidebarTrigger,
} from '@tb-dev/vue-components';

const {
  set,
  currentQuestion,
  active,
  isLoading,
  isLoadingSet,
  loadSet,
  startChunk,
  startRandomChunk,
  startRandomSource,
  startSource,
  startRandomSourceGroup,
  startSourceGroup,
} = useQuiz();

const isDialogTextChunkOpen = ref(false);

const isDialogSourcesOpen = ref(false);
const selectedSources = ref<SourceId[]>([]);

const isDialogSourceGroupsOpen = ref(false);
const selectedSourceGroups = ref<SourceGroupId[]>([]);

async function onSourceStart(ids: readonly SourceId[]) {
  if (ids.length > 0) {
    await startSource(ids);
  }
}

async function onSourceGroupStart(ids: readonly SourceGroupId[]) {
  if (ids.length > 0) {
    await startSourceGroup(ids);
  }
}
</script>

<template>
  <div class="flex size-full flex-col gap-2">
    <div class="flex h-14 w-full items-center justify-between px-2 md:px-6 py-4">
      <SidebarTrigger />

      <DialogTextChunk
        v-model:open="isDialogTextChunkOpen"
        :disabled="isLoading"
        @start="(chars) => startChunk(chars)"
      />

      <DialogSources
        v-model:open="isDialogSourcesOpen"
        v-model:selected="selectedSources"
        confirm-label="Start"
        :disabled="isLoading"
        disable-if-empty
        @confirm="onSourceStart"
      />

      <DialogSourceGroups
        v-model:open="isDialogSourceGroupsOpen"
        v-model:selected="selectedSourceGroups"
        confirm-label="Start"
        :disabled="isLoading"
        disable-if-empty
        @confirm="onSourceGroupStart"
      />

      <div class="flex items-center justify-center gap-2">
        <DropdownMenu v-if="!active">
          <DropdownMenuTrigger as-child>
            <Button size="sm" variant="default" :disabled="isLoading">
              <span>Menu</span>
            </Button>
          </DropdownMenuTrigger>

          <DropdownMenuContent side="top" class="min-w-(--reka-dropdown-menu-trigger-width)">
            <DropdownMenuGroup>
              <DropdownMenuButton :disabled="isLoading" @click="() => void (isDialogTextChunkOpen = true)">
                <span>Chunk</span>
              </DropdownMenuButton>

              <DropdownMenuButton :disabled="isLoading" @click="startRandomChunk">
                <span>Random Chunk</span>
              </DropdownMenuButton>

              <DropdownMenuButton :disabled="isLoading" @click="() => void (isDialogSourcesOpen = true)">
                <span>Source</span>
              </DropdownMenuButton>

              <DropdownMenuButton :disabled="isLoading" @click="startRandomSource">
                <span>Random Source</span>
              </DropdownMenuButton>

              <DropdownMenuButton :disabled="isLoading" @click="() => void (isDialogSourceGroupsOpen = true)">
                <span>Source Group</span>
              </DropdownMenuButton>

              <DropdownMenuButton :disabled="isLoading" @click="startRandomSourceGroup">
                <span>Random Source Group</span>
              </DropdownMenuButton>
            </DropdownMenuGroup>
          </DropdownMenuContent>
        </DropdownMenu>

        <Button
          v-if="!active"
          size="sm"
          variant="secondary"
          :disabled="isLoading"
          @click="loadSet"
        >
          <span>Reload</span>
        </Button>
      </div>
    </div>

    <div class="size-full flex flex-col gap-2 overflow-x-hidden overflow-y-auto px-1 md:px-6 pb-safe-12">
      <div v-if="isLoadingSet" class="size-full">
        <Loading />
      </div>
      <Chunks v-else-if="set && !active" :disabled="isLoading" />
      <Question v-else-if="currentQuestion" :disabled="isLoading" />
    </div>
  </div>
</template>
