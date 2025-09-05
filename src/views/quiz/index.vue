<script setup lang="ts">
import Set from './Set.vue';
import { getSet } from '@/commands';
import { useTemplateRef } from 'vue';
import Question from './Question.vue';
import { toPixel } from '@tb-dev/utils';
import { useQuiz } from '@/composables/quiz';
import { asyncRef, useHeightDiff } from '@tb-dev/vue';
import { Button, Loading } from '@tb-dev/vue-components';

const {
  state: set,
  execute: loadSet,
  isLoading: isLoadingSet,
} = asyncRef(null, getSet);

const {
  current,
  active,
  loading: isLoadingQuiz,
  start,
  answer,
  next,
  leave,
} = useQuiz();

const topbar = useTemplateRef('topbarEl');
const contentHeight = useHeightDiff(topbar);

async function onLoad() {
  await loadSet();
}

async function onLeave() {
  leave();
  await onLoad();
}
</script>

<template>
  <div class="flex size-full flex-col gap-2">
    <div ref="topbarEl" class="flex h-14 w-full items-center justify-end px-6 py-4">
      <Button
        size="sm"
        variant="secondary"
        :disabled="isLoadingSet || isLoadingQuiz"
        @click="onLoad"
      >
        <span>Reload</span>
      </Button>
    </div>

    <div
      class="flex flex-col gap-2 overflow-x-hidden overflow-y-auto px-6 pb-12"
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
        :current
        :disabled="isLoadingSet || isLoadingQuiz"
        @answer="(chosen) => answer(chosen)"
        @next="next"
        @leave="onLeave"
      />
    </div>
  </div>
</template>
