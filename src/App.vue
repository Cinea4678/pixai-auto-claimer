<script setup lang="ts">
import { onMounted, ref } from "vue"
import About from "./components/About.vue"
import { useI18n } from "vue-i18n"
import Main from "./components/Main.vue"
import { tauri } from "@tauri-apps/api"
import { store } from "./store/store.ts"

const showAbout = ref(false)

const languages = [
  { label: "English", key: "en" },
  { label: "简体中文", key: "zh" },
]

const i18n = useI18n()

const handleLanguageSelect = (language: string) => {
  i18n.locale.value = language
}

onMounted(async () => {
  tauri.invoke("get_job_state").then((job_state) => {
    store.state = job_state as APP.AppState
  })
})
</script>

<template>
  <n-dialog-provider>
    <n-message-provider>
      <div class="absolute top-0 left-0 p-3 w-full h-full flex flex-col gap-2">
        <div class="flex select-none">
          <div>
            <div class="font-mono text-5xl text-emerald-900">PixAI Auto Claimer</div>
            <div class="text-end">{{ $t("main.title") }}</div>
          </div>
          <div class="grow" />
          <div class="flex gap-2">
            <n-button @click="showAbout = true">{{ $t("main.about") }}</n-button>
            <n-dropdown trigger="click" :options="languages" @select="handleLanguageSelect">
              <n-button>语言/Language</n-button>
            </n-dropdown>
          </div>
        </div>

        <div class="mt-6 grow relative">
          <Main />
        </div>
      </div>

      <n-modal v-model:show="showAbout" preset="dialog" :show-icon="false" :title="$t('about.title')">
        <About />
      </n-modal>
    </n-message-provider>
  </n-dialog-provider>
</template>

<style scoped></style>
