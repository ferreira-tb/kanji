<script setup lang="ts">
import { useSettingsStore } from '@/stores/settings';
import {
  Checkbox,
  Label,
  NumberField,
  NumberFieldContent,
  NumberFieldDecrement,
  NumberFieldIncrement,
  NumberFieldInput,
} from '@tb-dev/vue-components';

const settings = useSettingsStore();
</script>

<template>
  <div class="flex size-full flex-col gap-6 p-4">
    <h1 class="text-2xl font-semibold">Settings</h1>
    <div class="flex flex-col size-full gap-4">
      <h2>General</h2>
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
    <div>
      <h2>Snippets</h2>
      <NumberField
        v-model="settings.snippetLimit"
        :min="1"
        :max="5_000"
        :step="1"
        class="mr-4"
      >
        <NumberFieldContent>
          <NumberFieldDecrement />
          <NumberFieldInput class="dark:bg-input/40" />
          <NumberFieldIncrement />
        </NumberFieldContent>
      </NumberField>
    </div>
  </div>
</template>
