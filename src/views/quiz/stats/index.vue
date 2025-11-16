<script setup lang="ts">
import { asyncRef } from '@tb-dev/vue';
import { formatPercent } from '@/lib/intl';
import { useQuiz } from '@/composables/quiz';
import { getQuizSourceStats } from '@/commands';
import { onActivated, onDeactivated } from 'vue';
import { useSources } from '@/composables/sources';
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

const {
  state: stats,
  execute: load,
  isLoading: isLoadingStats,
} = asyncRef([], getQuizSourceStats, { immediate: false });

const { set, isLoadingSet } = useQuiz();
const { findSource } = useSources();

const { isMobile } = useSidebar();

onActivated(load);

onDeactivated(() => void (stats.value = []));
</script>

<template>
  <div class="flex size-full flex-col gap-2">
    <div class="flex h-14 w-full items-center justify-between px-2 md:px-6 py-4">
      <SidebarTrigger />
      <div class="flex items-center justify-center gap-2">
        <Button size="sm" variant="secondary" :disabled="isLoadingStats" @click="load">
          <span>Reload</span>
        </Button>
      </div>
    </div>

    <div v-if="isLoadingSet || isLoadingStats" class="size-full">
      <Loading />
    </div>
    <template v-else>
      <div v-if="set && set.quizzes > 0" class="flex items-center justify-center px-2">
        <Card class="w-full md:max-w-96 p-0">
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

      <div class="overflow-x-hidden overflow-y-auto pb-safe-12 px-1 md:px-6">
        <div class="flex flex-col gap-4">
          <Table v-if="stats.length > 0">
            <TableHeader>
              <TableRow class="bg-background hover:bg-background">
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
    </template>
  </div>
</template>
