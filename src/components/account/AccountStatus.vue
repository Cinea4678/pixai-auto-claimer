<script setup lang="ts">
import { Spinner, HourglassHalf, Check, Times } from "@vicons/fa"
defineProps<{ status: number }>()
</script>

<template>
  <div class="flex justify-center items-center">
    <div v-if="status == 0">
      <n-icon size="12"><HourglassHalf /></n-icon>
    </div>
    <div v-if="status == 1">
      <n-icon size="12"><Spinner /></n-icon>
    </div>
    <div v-if="status == 2">
      <n-icon size="12"><Check /></n-icon>
    </div>
    <div v-if="status == 3">
      <n-icon size="12"><Times /></n-icon>
    </div>
  </div>
</template>

<style scoped></style>
