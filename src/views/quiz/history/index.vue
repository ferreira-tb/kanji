<script setup lang="ts">
import { useRouter } from 'vue-router';
import { toPixel } from '@tb-dev/utils';
import { formatZoned } from '@/lib/date';
import { getQuizAnswers } from '@/commands';
import { useSources } from '@/composables/sources';
import { asyncRef, useHeightDiff } from '@tb-dev/vue';
import { onActivated, onDeactivated, useTemplateRef } from 'vue';
import {
  Button,
  SidebarTrigger,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
  useSidebar,
} from '@tb-dev/vue-components';

const {
  state: answers,
  execute: load,
  isLoading,
} = asyncRef([], getQuizAnswers, { immediate: false });

const { findSource } = useSources();

const router = useRouter();
const { isMobile } = useSidebar();

const topbar = useTemplateRef('topbarEl');
const contentHeight = useHeightDiff(topbar);

onActivated(load);

onDeactivated(() => void (answers.value = []));
</script>

<template>
  <div class="flex size-full flex-col gap-2">
    <div ref="topbarEl" class="flex h-14 w-full items-center justify-between px-2 md:px-6 py-4">
      <SidebarTrigger />
      <div class="flex items-center justify-center gap-2">
        <Button size="sm" variant="secondary" :disabled="isLoading" @click="load">
          <span>Reload</span>
        </Button>
        <Button size="sm" variant="secondary" @click="() => router.back()">
          <span>Back</span>
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
            <TableHead>{{ isMobile ? 'Q' : 'Question' }}</TableHead>
            <TableHead>{{ isMobile ? 'A' : 'Answer' }}</TableHead>
            <TableHead>Source</TableHead>
          </TableRow>
        </TableHeader>

        <TableBody>
          <TableRow v-for="({ id, question, answer, sourceId, createdAt }) of answers" :key="id">
            <template v-if="sourceId">
              <TableCell>
                <span>{{ question }}</span>
              </TableCell>
              <TableCell :class="question === answer ? 'text-green-500' : 'text-red-500'">
                <span>{{ answer }}</span>
              </TableCell>
              <TableCell>
                <div class="flex flex-col">
                  <div class="text-muted-foreground text-xs">
                    <span>{{ formatZoned(createdAt, 'dd/MM HH:mm') }}</span>
                  </div>
                  <div class="break-all wrap-anywhere">
                    <span>{{ findSource(sourceId)?.name ?? '' }}</span>
                  </div>
                </div>
              </TableCell>
            </template>
          </TableRow>
        </TableBody>
      </Table>
    </div>
  </div>
</template>
