<script setup lang="ts">
import { store } from "../store/store.ts"
import dayjs from "dayjs"
</script>

<template>
  <div class="rounded-xl bg-white/65 flex flex-col items-center py-3 mx-4">
    <span>剩余任务</span>
    <span class="text-2xl">{{ store.state.jobs_left }}</span>
    <span>当前并行数量：{{ store.state.concurrent }}</span>
    <span>
      预计需要：{{
        store.state.time_left ? dayjs.duration(store.state.time_left!, "second").format("mm:ss") : "暂无剩余时间"
      }}
    </span>
  </div>
</template>

<style scoped></style>
