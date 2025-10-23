<script setup lang="ts">
import { toPixel } from '@tb-dev/utils';
import { formatZoned } from '@/lib/date';
import { getQuizAnswers } from '@/commands';
import { onActivated, useTemplateRef } from 'vue';
import { useSources } from '@/composables/sources';
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
} = asyncRef([], getQuizAnswers, { immediate: false });

const { findSource } = useSources();

const topbar = useTemplateRef('topbarEl');
const contentHeight = useHeightDiff(topbar);

onActivated(load);
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
      class="overflow-x-hidden overflow-y-auto pb-safe-12 px-1 md:px-6"
      :style="{ height: toPixel(contentHeight) }"
    >
      <Table v-if="answers.length > 0">
        <TableHeader>
          <TableRow>
            <TableHead>Question</TableHead>
            <TableHead>Answer</TableHead>
            <TableHead class="max-lg:hidden"></TableHead>
            <TableHead>Date</TableHead>
          </TableRow>
        </TableHeader>

        <TableBody>
          <TableRow v-for="({ id, question, answer, sourceId, createdAt }) of answers" :key="id">
            <TableCell>
              <span>{{ question }}</span>
            </TableCell>
            <TableCell :class="question === answer ? 'text-green-500' : 'text-red-500'">
              <span>{{ answer }}</span>
            </TableCell>
            <TableCell class="max-lg:hidden">
              <span v-if="sourceId">{{ findSource(sourceId)?.name ?? '' }}</span>
            </TableCell>
            <TableCell>
              <span>{{ formatZoned(createdAt, 'dd/MM HH:mm') }}</span>
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </div>
  </div>
</template>
