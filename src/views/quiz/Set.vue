<script setup lang="ts">
import { Button, Card, CardContent } from '@tb-dev/vue-components';

defineProps<{
  set: KanjiSet;
  disabled: boolean;
  onQuiz: (chunk: KanjiSetChunk) => Promise<void>;
}>();

function takeKanjis(chunk: KanjiSetChunk) {
  const kanjis: KanjiChar[] = [];
  for (let i = 0; i < 20; i++) {
    if (chunk.kanjis[i]) {
      kanjis.push(chunk.kanjis[i]);
    }
  }

  return kanjis;
}
</script>

<template>
  <Card v-for="chunk of set" :key="chunk.id" class="p-0">
    <CardContent class="flex justify-between items-center p-2 pr-8">
      <div class="flex flex-col gap-1">
        <h1>Chunk {{ chunk.id }}</h1>
        <h2 class="text-muted-foreground">
          <span>{{ takeKanjis(chunk).join('') }}</span>
        </h2>
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
    </CardContent>
  </Card>
</template>
