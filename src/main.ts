import { createApp } from "vue"
import "./styles.css"
import App from "./App.vue"
import messages from "./i18n/messages.ts"
import { createI18n } from "vue-i18n"

const i18n = createI18n({ messages, locale: navigator.language, fallbackLocale: "zh", legacy: false })

// Day.js
import dayjs from "dayjs"
import duration from "dayjs/plugin/duration"
dayjs.extend(duration)

// Mount NaiveUI After Tailwind CSS
const meta = document.createElement("meta")
meta.name = "naive-ui-style"
document.head.appendChild(meta)

createApp(App).use(i18n).mount("#app")
