<script setup lang="ts">
</script>

<template>
  <RouterView #default="{ Component }">
    <template v-if="Component">
      <KeepAlive>
        <component :is="Component" />
      </KeepAlive>
    </template>
  </RouterView>
</template>
