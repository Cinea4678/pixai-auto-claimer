<script setup lang="ts">
import { reactive, ref } from "vue"
import { store } from "../store/store.ts"
import { tauri } from "@tauri-apps/api"

const emits = defineEmits<{
  (e: "finish"): void
}>()

const formRef = ref()

const settings = reactive<APP.Settings>({
  concurrent: store.state.concurrent,
})
const loading = ref(false)

const handleSave = () => {
  loading.value = true
  tauri
    .invoke("set_settings", { settings })
    .then(() => {
      emits("finish")
    })
    .finally(() => {
      loading.value = false
    })
}
</script>

<template>
  <n-spin :show="loading">
    <n-form ref="formRef" :label-width="80" :model="settings" size="medium">
      <n-form-item label="并行数量（建议1~3）" path="email">
        <n-input-number
          v-model:value="settings!.concurrent"
          :min="1"
          :max="100"
          placeholder="请勿设置过高，否则可能导致未知行为"
        />
      </n-form-item>

      <div class="flex flex-row-reverse gap-2">
        <n-button attr-type="button" @click="handleSave" type="primary"> 保存 </n-button>
      </div>
    </n-form>
  </n-spin>
</template>

<style scoped></style>
