<script setup lang="ts">
import { formatPercent } from '@/lib/intl';
import { Button, Card, CardContent } from '@tb-dev/vue-components';

defineProps<{
  set: KanjiSet;
  disabled: boolean;
  onQuiz: (chunk: KanjiSetChunk) => Promise<void>;
}>();
</script>

<template>
  <Card v-for="chunk of set.chunks" :key="chunk.id" class="p-0">
    <CardContent class="p-2 pr-8">
      <div class="flex justify-between items-center gap-4">
        <div class="flex flex-col gap-1">
          <h1>Chunk {{ chunk.id }}</h1>
          <h2 class="text-muted-foreground">
            <span>{{ chunk.kanjis.join('') }}</span>
          </h2>
        </div>

        <div v-if="chunk.quizzes > 0" class="grid grid-cols-2 gap-4 px-4">
          <div class="flex h-16 flex-col items-center justify-center">
            <span class="text-muted-foreground text-sm">Quiz</span>
            <span class="text-lg font-semibold">{{ chunk.quizzes }}</span>
          </div>
          <div
            v-if="Number.isFinite(chunk.quizAccuracy)"
            class="flex h-16 flex-col items-center justify-center"
          >
            <span class="text-muted-foreground text-sm">Accuracy</span>
            <span class="text-lg font-semibold">
              {{ formatPercent(chunk.quizAccuracy) }}
            </span>
          </div>
        </div>

        <div>
          <Button
            size="sm"
            variant="default"
            :disabled
            class="px-4"
            @click="() => onQuiz(chunk)"
          >
            <span>Study</span>
          </Button>
        </div>
      </div>
    </CardContent>
  </Card>
</template>
