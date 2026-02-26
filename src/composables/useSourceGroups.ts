import { storeToRefs } from 'pinia';
import * as commands from '@/commands';
import { handleError } from '@/lib/error';
import { tryOnMounted } from '@vueuse/core';
import { useSettingsStore } from '@/stores/settings';
import { tryInjectOrElse, useMutex } from '@tb-dev/vue';
import { effectScope, type InjectionKey, markRaw, ref } from 'vue';

const SYMBOL = Symbol() as InjectionKey<ReturnType<typeof create>>;

export function useSourceGroups() {
  return tryInjectOrElse(SYMBOL, () => {
    const scope = effectScope(/* detached */ true);
    const value = scope.run(create)!;
    return {
      sourceGroups: value.sourceGroups,
      loading: value.loading,
      loadSourceGroups: value.loadSourceGroups,
      removeSourceGroup: value.removeSourceGroup,
      findSourceGroup: value.findSourceGroup,
    };
  });
}

function create() {
  const sourceGroups = ref<readonly SourceGroup[]>([]);
  const { locked, ...mutex } = useMutex();

  const settings = useSettingsStore();
  const { baseUrl } = storeToRefs(settings);

  async function loadSourceGroups() {
    try {
      await mutex.acquire();
      if (__DESKTOP__ || baseUrl.value) {
        sourceGroups.value = await commands.getSourceGroups()
          .then((it) => it.map(markRaw));
      }
      else {
        sourceGroups.value = [];
      }
    }
    catch (err) {
      handleError(err);
    }
    finally {
      mutex.release();
    }
  }

  async function removeSourceGroup(id: SourceGroupId) {
    try {
      await mutex.acquire();
      const rows = await commands.removeSourceGroup(id);
      if (rows > 0) {
        sourceGroups.value = sourceGroups.value.filter((group) => {
          return group.id !== id;
        });
      }
    }
    catch (err) {
      handleError(err);
    }
    finally {
      mutex.release();
    }
  }

  function findSourceGroup(id: SourceGroupId): Option<SourceGroup> {
    return sourceGroups.value.find((group) => group.id === id);
  }

  tryOnMounted(() => void loadSourceGroups());

  return {
    sourceGroups: sourceGroups as Readonly<typeof sourceGroups>,
    loading: locked,
    loadSourceGroups,
    removeSourceGroup,
    findSourceGroup,
  };
}
