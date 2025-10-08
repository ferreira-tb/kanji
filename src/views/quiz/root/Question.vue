<script setup lang="ts">
import { computed } from 'vue';
import * as commands from '@/commands';
import { StarIcon } from 'lucide-vue-next';
import { useQuiz } from '@/composables/quiz';
import { Badge, Button, Card, CardContent, cn } from '@tb-dev/vue-components';

const props = defineProps<{
  disabled: boolean;
}>();

const {
  quizSize,
  currentQuestion,
  currentIndex,
  chosenAnswer,
  canAnswer,
  ...quiz
} = useQuiz();

const source = computed(() => currentQuestion.value?.snippet.source);
const question = computed(() => {
  return canAnswer.value ?
    currentQuestion.value?.censored :
    currentQuestion.value?.snippet.content;
});

const isBookmarked = computed(() => {
  return typeof quiz.currentBookmark.value === 'number';
});

function getCardClass(option: KanjiChar) {
  let classList = 'p-2 md:p-4';
  if (canAnswer.value && !props.disabled) {
    classList += ' cursor-pointer hover:bg-accent';
  }

  if (currentQuestion.value && chosenAnswer.value && !canAnswer.value) {
    if (option === currentQuestion.value.answer) {
      classList += ' bg-green-300';
    }
    else if (option === chosenAnswer.value) {
      classList += ' bg-red-300';
    }
  }

  return cn(classList);
}

function getQuestionClass() {
  let classList = 'md:text-2xl select-text';
  if (question.value) {
    if (question.value.length > 300) {
      classList += 'text-base';
    }
    else if (question.value.length > 150) {
      classList += 'text-lg';
    }
    else {
      classList += 'text-xl';
    }
  }

  return cn(classList);
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

async function bookmark() {
  if (isBookmarked.value) {
    await quiz.removeBookmark();
  }
  else {
    await quiz.createBookmark();
  }
}
</script>

<template>
  <div class="size-full relative flex flex-col justify-center items-center p-6">
    <Badge v-if="currentIndex && quizSize" variant="outline" class="absolute top-2 left-2">
      <span class="text-muted-foreground text-xs">{{ `${currentIndex}/${quizSize}` }}</span>
    </Badge>

    <div class="absolute top-2 right-2">
      <Button variant="ghost" :disabled @click="bookmark">
        <StarIcon
          :fill="isBookmarked ? 'currentColor' : 'none'"
          class="size-4 lg:size-6 md:size-8"
        />
      </Button>
    </div>

    <div
      v-if="source && question"
      class="h-full flex flex-col justify-center items-center text-center gap-2"
    >
      <span class="cursor-pointer text-muted-foreground text-xs md:text-sm" @click="open">
        {{ source.name }}
      </span>
      <span :class="getQuestionClass()">
        {{ question }}
      </span>
    </div>

    <div v-if="currentQuestion" class="flex flex-col gap-8">
      <div class="grid grid-cols-5 gap-4">
        <div v-for="option of currentQuestion.options" :key="option">
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
