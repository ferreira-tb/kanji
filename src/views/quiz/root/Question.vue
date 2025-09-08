<script setup lang="ts">
import { computed } from 'vue';
import * as commands from '@/commands';
import { useQuiz } from '@/composables/quiz';
import { Button, Card, CardContent } from '@tb-dev/vue-components';

const props = defineProps<{
  disabled: boolean;
}>();

const { current, chosen, canAnswer, ...quiz } = useQuiz();

const source = computed(() => current.value?.snippet.source);
const question = computed(() => {
  return canAnswer.value ?
    current.value?.censored :
    current.value?.snippet.content;
});

function getCardClass(option: KanjiChar) {
  let classList = 'p-2 md:p-4';
  if (canAnswer.value && !props.disabled) {
    classList += ' cursor-pointer hover:bg-accent';
  }

  if (current.value && chosen.value && !canAnswer.value) {
    if (option === current.value.answer) {
      classList += ' bg-green-300';
    }
    else if (option === chosen.value) {
      classList += ' bg-red-300';
    }
  }

  return classList;
}

async function answer(option: KanjiChar) {
  if (canAnswer.value && !props.disabled) {
    await quiz.answer(option);
  }
}

function open() {
  if (source.value && !props.disabled) {
    commands.open(source.value.path, source.value.line).err();
  }
}
</script>

<template>
  <div class="size-full flex flex-col justify-center items-center p-6">
    <div
      v-if="source && question"
      class="h-full flex flex-col justify-center items-center text-center gap-2"
    >
      <span class="cursor-pointer text-muted-foreground text-sm" @click="open">
        {{ source.name }}
      </span>
      <span class="text-xl md:text-2xl select-text">
        {{ question }}
      </span>
    </div>

    <div v-if="current" class="flex flex-col gap-8">
      <div class="grid grid-cols-5 gap-4">
        <div v-for="option of current.options" :key="option">
          <Card
            :class="getCardClass(option)"
            @click="() => answer(option)"
          >
            <CardContent class="p-2 md:p-4">
              <div class="flex justify-center items-center">
                <span class="text-xl md:text-4xl font-bold">{{ option }}</span>
              </div>
            </CardContent>
          </Card>
        </div>
      </div>

      <div class="flex justify-center items-center">
        <div class="grid grid-cols-2 w-max justify-center items-center gap-4">
          <Button
            variant="default"
            size="lg"
            :disabled="canAnswer || disabled"
            class="max-w-24 px-6"
            @click="() => quiz.next()"
          >
            <span>Next</span>
          </Button>

          <Button
            variant="secondary"
            size="lg"
            :disabled
            class="max-w-24 px-6"
            @click="() => quiz.leave()"
          >
            <span>Leave</span>
          </Button>
        </div>
      </div>
    </div>
  </div>
</template>
