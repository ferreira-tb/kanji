<script setup lang="ts">
import * as commands from '@/commands';
import { computed, nextTick } from 'vue';
import type { Option } from '@tb-dev/utils';
import { Button, Card, CardContent } from '@tb-dev/vue-components';

const props = defineProps<{
  current: QuizQuestion;
  disabled: boolean;
  onAnswer: () => Promise<void>;
  onNext: () => void;
  onLeave: () => void;
}>();

const chosen = defineModel<Option<KanjiChar>>('chosen', { required: true });
const canAnswer = defineModel<boolean>('canAnswer', { required: true });

const source = computed(() => props.current.snippet.source);
const question = computed(() => {
  return canAnswer.value ? props.current.censored : props.current.snippet.content;
});

async function answer(option: KanjiChar) {
  if (canAnswer.value && !props.disabled) {
    canAnswer.value = false;
    chosen.value = option;
    await nextTick();
    await props.onAnswer();
  }
}

async function next() {
  if (!canAnswer.value && !props.disabled) {
    canAnswer.value = true;
    chosen.value = null;
    await nextTick();
    props.onNext();
  }
}

function leave() {
  chosen.value = null;
  props.onLeave();
}

function getCardClass(option: KanjiChar) {
  let classList = 'p0 md:p-4';
  if (canAnswer.value && !props.disabled) {
    classList += ' cursor-pointer hover:bg-accent';
  }

  if (!canAnswer.value && chosen.value) {
    if (option === props.current.answer) {
      classList += ' bg-green-300';
    }
    else if (option === chosen.value) {
      classList += ' bg-red-300';
    }
  }

  return classList;
}
</script>

<template>
  <div class="size-full flex flex-col justify-center items-center p-6">
    <div class="h-full flex flex-col justify-center items-center text-center gap-2">
      <span
        class="cursor-pointer text-muted-foreground text-sm"
        @click="() => commands.open(source.path, source.line)"
      >
        {{ source.name }}
      </span>
      <span class="text-xl md:text-2xl select-text">
        {{ question }}
      </span>
    </div>

    <div class="flex flex-col gap-8">
      <div class="grid grid-cols-5 gap-4">
        <div v-for="option of current.options" :key="option">
          <Card :class="getCardClass(option)" class="p-2" @click="() => answer(option)">
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
            @click="next"
          >
            <span>Next</span>
          </Button>

          <Button
            variant="secondary"
            size="lg"
            :disabled
            class="max-w-24 px-6"
            @click="leave"
          >
            <span>Leave</span>
          </Button>
        </div>
      </div>
    </div>
  </div>
</template>
