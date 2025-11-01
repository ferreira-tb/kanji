<script setup lang="ts">
import { go } from '@/router';
import Chunks from './Chunks.vue';
import Question from './Question.vue';
import { toPixel } from '@tb-dev/utils';
import { ref, useTemplateRef } from 'vue';
import { useHeightDiff } from '@tb-dev/vue';
import { useQuiz } from '@/composables/quiz';
import DialogSource from './DialogSource.vue';
import { useSources } from '@/composables/sources';
import DropdownMenuButton from '@/components/DropdownMenuButton.vue';
import {
  Button,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuSeparator,
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
  startRandomChunk,
  startRandomSource,
} = useQuiz();

const { sources } = useSources();

const topbar = useTemplateRef('topbarEl');
const contentHeight = useHeightDiff(topbar);

const isSourceDialogOpen = ref(false);
</script>

<template>
  <div class="flex size-full flex-col gap-2">
    <div ref="topbarEl" class="flex h-14 w-full items-center justify-between px-2 md:px-6 py-4">
      <SidebarTrigger />
      <DialogSource v-model="isSourceDialogOpen" />

      <div class="flex items-center justify-center gap-2">
        <DropdownMenu v-if="!active">
          <DropdownMenuTrigger as-child>
            <Button size="sm" variant="default" :disabled="isLoading">
              <span>Menu</span>
            </Button>
          </DropdownMenuTrigger>

          <DropdownMenuContent side="top" class="w-(--reka-dropdown-menu-trigger-width)">
            <DropdownMenuGroup>
              <DropdownMenuButton
                :disabled="isLoading || sources.length === 0"
                @click="() => void (isSourceDialogOpen = true)"
              >
                <span>Source</span>
              </DropdownMenuButton>

              <DropdownMenuButton
                :disabled="isLoading || sources.length === 0"
                @click="startRandomSource"
              >
                <span>Random Source</span>
              </DropdownMenuButton>

              <DropdownMenuButton :disabled="isLoading" @click="startRandomChunk">
                <span>Random Chunk</span>
              </DropdownMenuButton>
            </DropdownMenuGroup>

            <DropdownMenuSeparator />

            <DropdownMenuButton @click="() => go('quiz-stats')">
              <span>Stats</span>
            </DropdownMenuButton>
            <DropdownMenuButton @click="() => go('quiz-history')">
              <span>History</span>
            </DropdownMenuButton>
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

    <div
      class="flex flex-col gap-2 overflow-x-hidden overflow-y-auto px-1 md:px-6 pb-safe-12"
      :style="{ height: toPixel(contentHeight) }"
    >
      <div v-if="isLoadingSet" class="size-full">
        <Loading />
      </div>
      <Chunks v-else-if="set && !active" :disabled="isLoading" />
      <Question v-else-if="currentQuestion" :disabled="isLoading" />
    </div>
  </div>
</template>
