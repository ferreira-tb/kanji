<script setup lang="ts">
import * as commands from '@/commands';
import { sinceZoned } from '@/lib/date';
import { computed, onMounted } from 'vue';
import { formatPercent } from '@/lib/intl';
import { useBreakpoints } from '@tb-dev/vue';
import { useQuiz } from '@/composables/useQuiz';
import { useQuizChunkHistory } from '@/composables/useQuizChunkHistory';
import { Badge, Button, Card, CardContent } from '@tb-dev/vue-components';

defineProps<{
  disabled: boolean;
}>();

const { set, startChunk } = useQuiz();

const chunks = computed(() => set.value?.chunks ?? []);

const { md } = useBreakpoints();

const { quizChunkHistory, loadQuizChunkHistoryEntries } = useQuizChunkHistory();

async function start(chunk: KanjiSetChunk) {
  if (chunk.kanjis.length > 0) {
    await commands.createQuizChunkHistoryEntry(chunk.id);
    await startChunk(chunk.kanjis);
  }
}

onMounted(() => loadQuizChunkHistoryEntries().err());
</script>

<template>
  <Card v-for="chunk of chunks" :key="chunk.id" class="p-0">
    <CardContent class="p-2 md:pr-8">
      <div class="flex justify-between items-center gap-4 md:gap-8 xl:gap-12">
        <div class="w-full flex justify-between items-center gap-4">
          <div class="flex flex-col gap-0.5 md:gap-1">
            <div class="flex justify-start items-center gap-2">
              <h1>Chunk {{ chunk.id }}</h1>
              <Badge v-if="md && quizChunkHistory.has(chunk.id)" variant="outline">
                <span>{{ sinceZoned(quizChunkHistory.get(chunk.id)!.lastQuiz) }}</span>
              </Badge>
            </div>

            <h2 class="text-muted-foreground">
              <span v-if="md">{{ chunk.kanjis.join('') }}</span>
              <span v-else-if="quizChunkHistory.has(chunk.id)" class="text-xs">
                {{ sinceZoned(quizChunkHistory.get(chunk.id)!.lastQuiz) }}
              </span>
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
