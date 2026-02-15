<script setup lang="ts">
import { asyncRef } from '@tb-dev/vue';
import { formatZoned } from '@/lib/date';
import { getQuizAnswers } from '@/commands';
import { onActivated, onDeactivated } from 'vue';
import { useSources } from '@/composables/useSources';
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
  load,
  loading,
} = asyncRef([], getQuizAnswers, { immediate: false });

const { findSource } = useSources();

const { isMobile } = useSidebar();

onActivated(load);

onDeactivated(() => void (answers.value = []));
</script>

<template>
  <div class="flex size-full flex-col gap-2">
    <div class="flex h-14 w-full items-center justify-between px-2 md:px-6 py-4">
      <SidebarTrigger />
      <div class="flex items-center justify-center gap-2">
        <Button size="sm" variant="secondary" :disabled="loading" @click="load">
          <span>Reload</span>
        </Button>
      </div>
    </div>

    <div class="overflow-x-hidden overflow-y-auto pb-safe-12 px-1 md:px-6">
      <Table v-if="answers.length > 0">
        <TableHeader>
          <TableRow class="bg-background hover:bg-background">
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
                <div class="flex flex-col gap-0.5">
                  <div class="text-muted-foreground text-xs">
                    {{ formatZoned(createdAt, 'dd/MM HH:mm') }}
                  </div>
                  <div class="break-all wrap-anywhere">
                    {{ findSource(sourceId)?.name ?? '' }}
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
