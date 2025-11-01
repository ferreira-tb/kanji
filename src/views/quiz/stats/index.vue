<script setup lang="ts">
import { useRouter } from 'vue-router';
import { toPixel } from '@tb-dev/utils';
import { formatPercent } from '@/lib/intl';
import { useQuiz } from '@/composables/quiz';
import { getQuizSourceStats } from '@/commands';
import { useSources } from '@/composables/sources';
import { asyncRef, useHeightDiff } from '@tb-dev/vue';
import { onActivated, onDeactivated, useTemplateRef } from 'vue';
import {
  Button,
  Card,
  CardContent,
  Loading,
  SidebarTrigger,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
  useSidebar,
} from '@tb-dev/vue-components';

const { set, isLoadingSet } = useQuiz();
const { findSource } = useSources();

const {
  state: stats,
  execute: load,
  isLoading: isLoadingStats,
} = asyncRef([], getQuizSourceStats, { immediate: false });

const router = useRouter();
const { isMobile } = useSidebar();

const topbar = useTemplateRef('topbarEl');
const contentHeight = useHeightDiff(topbar);

onActivated(load);

onDeactivated(() => void (stats.value = []));
</script>

<template>
  <div class="flex size-full flex-col gap-2">
    <div ref="topbarEl" class="flex h-14 w-full items-center justify-between px-2 md:px-6 py-4">
      <SidebarTrigger />
      <div class="flex items-center justify-center gap-2">
        <Button size="sm" variant="secondary" :disabled="isLoadingStats" @click="load">
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
      <div v-if="isLoadingSet || isLoadingStats" class="size-full">
        <Loading />
      </div>
      <div v-else class="flex flex-col gap-4">
        <div class="flex items-center justify-center">
          <Card v-if="set && set.quizzes > 0" class="w-full md:max-w-96 p-0">
            <CardContent class="p-2 md:pr-8">
              <div class="grid grid-cols-2 gap-4 px-2 md:px-4">
                <div class="flex h-16 flex-col items-center justify-center">
                  <span class="text-muted-foreground text-xs md:text-sm">Quizzes</span>
                  <span class="text-base md:text-lg font-semibold">{{ set.quizzes }}</span>
                </div>
                <div
                  v-if="Number.isFinite(set.quizAccuracy)"
                  class="flex h-16 flex-col items-center justify-center"
                >
                  <span class="text-muted-foreground text-xs md:text-sm">Accuracy</span>
                  <span class="text-base md:text-lg font-semibold">
                    {{ formatPercent(set.quizAccuracy) }}
                  </span>
                </div>
              </div>
            </CardContent>
          </Card>
        </div>

        <Table v-if="stats.length > 0">
          <TableHeader>
            <TableRow>
              <TableHead>Source</TableHead>
              <TableHead>{{ isMobile ? 'Q' : 'Quizzes' }}</TableHead>
              <TableHead>{{ isMobile ? '%' : 'Accuracy' }}</TableHead>
            </TableRow>
          </TableHeader>

          <TableBody>
            <TableRow v-for="({ source, quizzes, quizAccuracy }) of stats" :key="source">
              <TableCell class="break-all wrap-anywhere">
                <span>{{ findSource(source)?.name ?? '' }}</span>
              </TableCell>
              <TableCell>
                <span>{{ quizzes }}</span>
              </TableCell>
              <TableCell>
                <span>{{ formatPercent(quizAccuracy) }}</span>
              </TableCell>
            </TableRow>
          </TableBody>
        </Table>
      </div>
    </div>
  </div>
</template>
