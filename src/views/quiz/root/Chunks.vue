<script setup lang="ts">
import { computed } from 'vue';
import { since } from '@/lib/date';
import { localRef } from '@tb-dev/vue';
import { formatPercent } from '@/lib/intl';
import { useQuiz } from '@/composables/quiz';
import { Badge, Button, Card, CardContent } from '@tb-dev/vue-components';

defineProps<{
  disabled: boolean;
}>();

const { set, startChunk } = useQuiz();

const chunks = computed(() => set.value?.chunks ?? []);

interface LastChunk {
  readonly id: KanjiSetChunkId;
  readonly date: number;
}

const last = localRef<Option<LastChunk>>('kanji:quiz-last-chunk', null, {
  deep: true,
  initOnMounted: true,
  listenToStorageChanges: false,
  writeDefaults: true,
});

async function start(chunk: KanjiSetChunk) {
  if (chunk.kanjis.length > 0) {
    last.value = { id: chunk.id, date: Date.now() };
    await startChunk(chunk.kanjis);
  }
}
</script>

<template>
  <Card v-for="chunk of chunks" :key="chunk.id" class="p-0">
    <CardContent class="p-2 md:pr-8">
      <div class="flex justify-between items-center gap-4 md:gap-8 xl:gap-12">
        <div class="w-full flex justify-between items-center gap-4">
          <div class="flex flex-col gap-1">
            <div class="flex justify-start items-center gap-2">
              <h1>Chunk {{ chunk.id }}</h1>
              <Badge v-if="last && chunk.id === last.id" variant="outline">
                <span>{{ since(last.date) }}</span>
              </Badge>
            </div>
            <h2 class="hidden md:block text-muted-foreground">
              <span>{{ chunk.kanjis.join('') }}</span>
            </h2>
          </div>

          <div v-if="chunk.quizzes > 0" class="grid grid-cols-2 gap-4 px-2 md:px-4">
            <div class="flex h-16 flex-col items-center justify-center">
              <span class="text-muted-foreground text-xs md:text-sm">Quiz</span>
              <span class="text-base md:text-lg font-semibold">{{ chunk.quizzes }}</span>
            </div>
            <div
              v-if="Number.isFinite(chunk.quizAccuracy)"
              class="flex h-16 flex-col items-center justify-center"
            >
              <span class="text-muted-foreground text-xs md:text-sm">Accuracy</span>
              <span class="text-base md:text-lg font-semibold">
                {{ formatPercent(chunk.quizAccuracy) }}
              </span>
            </div>
          </div>
        </div>

        <div>
          <Button
            size="sm"
            variant="default"
            :disabled
            class="px-4"
            @click="() => start(chunk)"
          >
            <span>Start</span>
          </Button>
        </div>
      </div>
    </CardContent>
  </Card>
</template>
