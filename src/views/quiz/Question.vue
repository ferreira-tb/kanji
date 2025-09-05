<script setup lang="ts">
import { open } from '@/commands';
import { computed, ref } from 'vue';
import type { Option } from '@tb-dev/utils';
import { Button, Card, CardContent } from '@tb-dev/vue-components';

const props = defineProps<{
  current: QuizQuestion;
  disabled: boolean;
  onAnswer: (chosen: string) => Promise<void>;
  onNext: () => void;
  onLeave: () => Promise<void>;
}>();

const ready = ref(true);
const chosen = ref<Option<KanjiChar>>();

const source = computed(() => props.current.snippet.source);
const question = computed(() => {
  return ready.value ? props.current.censored : props.current.snippet.content;
});

async function answer(option: KanjiChar) {
  if (ready.value && !props.disabled) {
    ready.value = false;
    chosen.value = option;
    await props.onAnswer(option);
  }
}

function next() {
  if (!ready.value && !props.disabled) {
    ready.value = true;
    chosen.value = null;
    props.onNext();
  }
}

function leave() {
  chosen.value = null;
  props.onLeave().err();
}

function getCardClass(option: KanjiChar) {
  let classList = 'p-4';
  if (ready.value && !props.disabled) {
    classList += ' cursor-pointer hover:bg-accent';
  }

  if (!ready.value && chosen.value) {
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
    <div class="h-full flex flex-col justify-center items-center text-2xl text-center gap-2">
      <span
        class="cursor-pointer text-muted-foreground text-sm"
        @click="() => open(source.path, source.line)"
      >
        {{ source.name }}
      </span>
      <span class="select-text">{{ question }}</span>
    </div>

    <div class="flex flex-col gap-8">
      <div class="grid grid-cols-5 gap-4">
        <div v-for="option of current.options" :key="option">
          <Card :class="getCardClass(option)" @click="() => answer(option)">
            <CardContent>
              <div class="flex justify-center items-center">
                <span class="text-4xl font-bold">{{ option }}</span>
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
            :disabled="ready || disabled"
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
