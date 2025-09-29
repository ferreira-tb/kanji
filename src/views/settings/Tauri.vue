<script setup lang="ts">
import { useSettingsStore } from '@/stores/settings';
import {
  Checkbox,
  Input,
  Label,
  NumberField,
  NumberFieldContent,
  NumberFieldDecrement,
  NumberFieldIncrement,
  NumberFieldInput,
  Separator,
} from '@tb-dev/vue-components';

const settings = useSettingsStore();
</script>

<template>
  <div class="flex flex-col gap-4">
    <h2 class="text-xl">General</h2>
    <div class="flex flex-col gap-2">
      <Label>
        <Checkbox v-model="settings.clipboard" />
        <span>Copy to clipboard</span>
      </Label>
      <Label>
        <Checkbox v-model="settings.hideOnClose" />
        <span>Hide on close</span>
      </Label>
    </div>
  </div>

  <Separator />

  <div class="flex flex-col gap-4">
    <h2 class="text-xl">Snippets</h2>
    <Label>
      <Checkbox v-model="settings.shuffleSnippets" />
      <span>Shuffle snippets</span>
    </Label>

    <Label class="max-w-70">
      <span>Limit</span>
      <NumberField
        v-model="settings.snippetLimit"
        :min="1"
        :max="5_000"
        :step="1"
        class="w-full"
      >
        <NumberFieldContent>
          <NumberFieldDecrement />
          <NumberFieldInput class="dark:bg-input/40" />
          <NumberFieldIncrement />
        </NumberFieldContent>
      </NumberField>
    </Label>

    <Label class="max-w-70">
      <span>Minimum length</span>
      <NumberField
        v-model="settings.snippetMinLen"
        :min="1"
        :max="250"
        :step="1"
        class="w-full"
      >
        <NumberFieldContent>
          <NumberFieldDecrement />
          <NumberFieldInput class="dark:bg-input/40" />
          <NumberFieldIncrement />
        </NumberFieldContent>
      </NumberField>
    </Label>

    <Label class="max-w-70">
      <span>Kanji threshold</span>
      <NumberField
        v-model="settings.snippetKanjiThreshold"
        :min="0.1"
        :max="1"
        :step="0.05"
        class="w-full"
      >
        <NumberFieldContent>
          <NumberFieldDecrement />
          <NumberFieldInput class="dark:bg-input/40" />
          <NumberFieldIncrement />
        </NumberFieldContent>
      </NumberField>
    </Label>
  </div>

  <Separator />

  <div class="flex flex-col gap-4">
    <h2 class="text-xl">Set</h2>
    <Label class="max-w-70">
      <span>File name</span>
      <Input v-model="settings.kanjiSetFileName" type="text" />
    </Label>

    <Label class="max-w-70">
      <span>Size</span>
      <NumberField
        v-model="settings.kanjiSetSize"
        :min="10"
        :max="100"
        :step="1"
        class="w-full"
      >
        <NumberFieldContent>
          <NumberFieldDecrement />
          <NumberFieldInput class="dark:bg-input/40" />
          <NumberFieldIncrement />
        </NumberFieldContent>
      </NumberField>
    </Label>
  </div>
</template>
