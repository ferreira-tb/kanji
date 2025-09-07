<script setup lang="ts">
</script>

<template>
  <RouterView #default="{ Component }">
    <template v-if="Component">
      <component :is="Component" />
    </template>
  </RouterView>
</template>
