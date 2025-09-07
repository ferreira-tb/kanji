<script setup lang="ts">
import { useTemplateRef } from 'vue';
import * as commands from '@/commands';
import { toPixel } from '@tb-dev/utils';
import { formatZoned } from '@/lib/date';
import { asyncRef, useHeightDiff } from '@tb-dev/vue';
import {
  Button,
  SidebarTrigger,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@tb-dev/vue-components';

const {
  state: answers,
  execute: load,
  isLoading,
} = asyncRef([], commands.getQuizAnswers);

const topbar = useTemplateRef('topbarEl');
const contentHeight = useHeightDiff(topbar);

function format(answer: QuizAnswer) {
  return formatZoned(answer.createdAt, 'dd/MM HH:mm');
}
</script>

<template>
  <div class="flex size-full flex-col gap-2">
    <div ref="topbarEl" class="flex h-14 w-full items-center justify-between px-2 md:px-6 py-4">
      <SidebarTrigger />
      <div class="flex items-center justify-center gap-2">
        <Button
          size="sm"
          variant="secondary"
          :disabled="isLoading"
          @click="load"
        >
          <span>Reload</span>
        </Button>
      </div>
    </div>

    <div
      class="overflow-x-hidden overflow-y-auto pb-12 px-1 md:px-6"
      :style="{ height: toPixel(contentHeight) }"
    >
      <Table v-if="answers.length > 0">
        <TableHeader>
          <TableRow>
            <TableHead>Question</TableHead>
            <TableHead>Answer</TableHead>
            <TableHead>Date</TableHead>
          </TableRow>
        </TableHeader>

        <TableBody>
          <TableRow v-for="answer of answers" :key="answer.id">
            <TableCell>{{ answer.question }}</TableCell>
            <TableCell>{{ answer.answer }}</TableCell>
            <TableCell>{{ format(answer) }}</TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </div>
  </div>
</template>
