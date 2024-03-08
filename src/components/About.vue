<script setup lang="ts">
import { useI18n } from "vue-i18n"

const i18n = useI18n()
</script>

<template>
  <div v-if="i18n.locale.value.startsWith('en')">
    <p>Developed by Cinea（<a href="https://www.cinea.cc" target="_blank" class="text-green-700">cinea.cc</a>）</p>
    <p>This software is not for-profit or paid.</p>
  </div>
  <div v-else>
    <p>开发：Cinea（<a href="https://www.cinea.cc" target="_blank" class="text-green-700">cinea.cc</a>）</p>
    <p>本软件不是盈利和付费软件。</p>
  </div>
</template>

<style scoped></style>
