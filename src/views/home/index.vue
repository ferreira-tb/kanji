<script setup lang="ts">
import { chunk } from 'es-toolkit';
import { storeToRefs } from 'pinia';
import * as commands from '@/commands';
import { toPixel } from '@tb-dev/utils';
import { handleError } from '@/lib/error';
import Search from '@/components/Search.vue';
import { useKanjiStore } from '@/stores/kanji';
import { useKanjis } from '@/composables/kanji';
import { useSettingsStore } from '@/stores/settings';
import { useHeight, useHeightDiff } from '@tb-dev/vue';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { computed, type DeepReadonly, ref, useTemplateRef } from 'vue';
import {
  Button,
  Card,
  CardContent,
  Pagination,
  PaginationEllipsis,
  PaginationList,
  PaginationListItem,
  SidebarTrigger,
  useSidebar,
} from '@tb-dev/vue-components';

const store = useKanjiStore();
const { search, currentKanji } = storeToRefs(store);

const settings = useSettingsStore();

const { kanjis, loading, load } = useKanjis();

const { isMobile, ...sidebar } = useSidebar();

const currentPage = ref(1);
const itemsPerPage = ref(200);
const isPaginationEnabled = computed(() => {
  return isMobile.value && kanjis.value.length > 1000;
});

const chunks = computed(() => {
  if (isPaginationEnabled.value) {
    const map = new Map<number, KanjiStats[]>();
    const arrays = chunk(kanjis.value, itemsPerPage.value);
    arrays.forEach((array, index) => map.set(index + 1, array));
    return map;
  }
  else {
    return null;
  }
});

const currentChunk = computed(() => {
  if (chunks.value) {
    return chunks.value.get(currentPage.value) ?? [];
  }
  else {
    return kanjis.value;
  }
});

const desktop = globalThis.__DESKTOP__;
const topbar = useTemplateRef('topbarEl');
const contentHeight = useHeightDiff(topbar);

const pagination = useTemplateRef('pagination');
const paginationHeight = useHeight(pagination);

const gridMaxHeight = computed(() => {
  let height = contentHeight.value;
  if (isPaginationEnabled.value) {
    height -= paginationHeight.value;
  }

  return Math.max(height, 0);
});

async function addSource() {
  try {
    await commands.createSource();
    await load();
  }
  catch (err) {
    handleError(err);
  }
}

function onCardClick(kanji: DeepReadonly<KanjiStats>) {
  currentKanji.value = kanji;
  if (isMobile.value) {
    sidebar.setOpenMobile(true);
  }

  if (desktop && settings.clipboard) {
    writeText(kanji.character).err();
  }
}
</script>

<template>
  <div class="relative flex size-full flex-col gap-2">
    <div ref="topbarEl" class="flex h-14 w-full items-center justify-between gap-4 px-2 md:px-6 py-4">
      <div class="flex items-center justify-center gap-2">
        <SidebarTrigger />
        <Search v-model="search" class="hidden lg:block" />
      </div>

      <div class="flex items-center justify-center gap-4">
        <div class="flex items-center justify-center gap-2">
          <Button
            v-if="desktop"
            size="sm"
            :disabled="loading"
            @click="addSource"
          >
            <span>Add</span>
          </Button>

          <Button
            size="sm"
            variant="secondary"
            :disabled="loading"
            @click="load"
          >
            <span>Reload</span>
          </Button>

          <Button
            v-if="desktop"
            size="sm"
            variant="secondary"
            :disabled="loading"
            @click="() => commands.exportSet().err()"
          >
            <span>Export</span>
          </Button>
        </div>
      </div>
    </div>

    <div id="kanji-grid-container" class="top-14! pb-safe" :style="{ height: toPixel(contentHeight) }">
      <div
        v-if="currentChunk.length > 0"
        id="kanji-grid"
        :style="{ maxHeight: toPixel(gridMaxHeight) }"
        class="grid-cols-4 sm:grid-cols-6 lg:grid-cols-10 2xl:grid-cols-12 px-1 md:px-4"
      >
        <Card v-for="kanji of currentChunk" :key="kanji.character" class="p-2">
          <CardContent>
            <div
              class="flex cursor-pointer flex-col items-center"
              @click="() => onCardClick(kanji)"
            >
              <span class="text-3xl font-bold">{{ kanji.character }}</span>
              <span class="text-muted-foreground text-xs">{{ kanji.seen }}</span>
            </div>
          </CardContent>
        </Card>
      </div>

      <div
        v-if="isPaginationEnabled"
        ref="pagination"
        class="py-3 flex justify-center"
      >
        <Pagination
          #default="{ page }"
          v-model:page="currentPage"
          :total="kanjis.length"
          :items-per-page
          :sibling-count="1"
          show-edges
        >
          <PaginationList #default="{ items }" class="flex items-center gap-1">
            <template v-for="item of items">
              <PaginationListItem
                v-if="item.type === 'page'"
                :key="item.value"
                :value="item.value"
                as-child
              >
                <Button
                  :variant="item.value === page ? 'default' : 'outline'"
                  size="sm"
                  class="size-8 p-0"
                >
                  <span>{{ item.value }}</span>
                </Button>
              </PaginationListItem>
              <PaginationEllipsis v-else :key="item.type" />
            </template>
          </PaginationList>
        </Pagination>
      </div>
    </div>
  </div>
</template>

<style scoped>
#kanji-grid-container {
  display: flex;
  position: absolute;
  inset: 0;
  flex-direction: column;
  justify-content: space-between;
  overflow: hidden;
}

#kanji-grid {
  display: grid;
  gap: 0.5rem;
  overflow-x: hidden;
  overflow-y: auto;
  padding-bottom: v-bind("isPaginationEnabled ? '0px': '2rem'");
}
</style>
