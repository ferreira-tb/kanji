<script setup lang="ts">
import { go } from '@/router';
import Chunks from './Chunks.vue';
import { useTemplateRef } from 'vue';
import Question from './Question.vue';
import { toPixel } from '@tb-dev/utils';
import { useHeightDiff } from '@tb-dev/vue';
import { useQuiz } from '@/composables/quiz';
import { Button, Loading, SidebarTrigger } from '@tb-dev/vue-components';

const {
  set,
  currentQuestion,
  active,
  isLoading,
  isLoadingSet,
  loadSet,
  startRandom,
} = useQuiz();

const topbar = useTemplateRef('topbarEl');
const contentHeight = useHeightDiff(topbar);
</script>

<template>
  <div class="flex size-full flex-col gap-2">
    <div ref="topbarEl" class="flex h-14 w-full items-center justify-between px-2 md:px-6 py-4">
      <SidebarTrigger />
      <div class="flex items-center justify-center gap-2">
        <Button
          v-if="!active"
          size="sm"
          variant="default"
          :disabled="isLoading"
          @click="startRandom"
        >
          <span>Random</span>
        </Button>

        <Button
          v-if="!active"
          size="sm"
          variant="secondary"
          :disabled="isLoading"
          @click="loadSet"
        >
          <span>Reload</span>
        </Button>

        <Button
          size="sm"
          variant="secondary"
          role="link"
          tabindex="0"
          @click="() => go('quiz-history')"
        >
          <span>History</span>
        </Button>
      </div>
    </div>

    <div
      class="flex flex-col gap-2 overflow-x-hidden overflow-y-auto px-1 md:px-6 pb-12"
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
