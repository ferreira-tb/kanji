<script setup lang="ts">
import Set from './Set.vue';
import { go } from '@/router';
import { useTemplateRef } from 'vue';
import Question from './Question.vue';
import * as commands from '@/commands';
import { toPixel } from '@tb-dev/utils';
import { useQuiz } from '@/composables/quiz';
import { asyncRef, useHeightDiff } from '@tb-dev/vue';
import { Button, Loading, SidebarTrigger } from '@tb-dev/vue-components';

const {
  state: set,
  execute: loadSet,
  isLoading: isLoadingSet,
} = asyncRef(null, commands.getSet);

const {
  current,
  active,
  loading: isLoadingQuiz,
  start,
} = useQuiz(loadSet);

const topbar = useTemplateRef('topbarEl');
const contentHeight = useHeightDiff(topbar);

async function onLoad() {
  await loadSet();
}
</script>

<template>
  <div class="flex size-full flex-col gap-2">
    <div ref="topbarEl" class="flex h-14 w-full items-center justify-between px-2 md:px-6 py-4">
      <SidebarTrigger />
      <div class="flex items-center justify-center gap-2">
        <Button
          v-if="!active"
          size="sm"
          variant="secondary"
          :disabled="isLoadingSet || isLoadingQuiz"
          @click="onLoad"
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
      <Set
        v-else-if="set && !active"
        :set
        :disabled="isLoadingSet || isLoadingQuiz"
        @quiz="(chunk) => start(chunk.kanjis)"
      />
      <Question
        v-else-if="current"
        :disabled="isLoadingSet || isLoadingQuiz"
        :load-set
      />
    </div>
  </div>
</template>
