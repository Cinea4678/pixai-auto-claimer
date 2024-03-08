<script setup lang="ts">
import { Plus } from "@vicons/fa"
import { computed, onMounted, ref } from "vue"
import JobInfo from "./JobInfo.vue"
import EditAccount from "./account/EditAccount.vue"
import { tauri } from "@tauri-apps/api"
import { store } from "../store/store.ts"
import { listen } from "@tauri-apps/api/event"
import { useMessage } from "naive-ui"
import AccountStatus from "./account/AccountStatus.vue"
import Settings from "./Settings.vue"

const message = useMessage()

const disabled = computed(() => store.state.running)

const accounts = ref<APP.PixAiAccount[]>([{ email: "test@mail.io", password: "1234567", status: "" }])
const showPassword = ref(false)
const showEdit = ref(false)
const showEditDelete = ref(false)
const editTitle = ref("编辑账号信息")
const curAccount = ref<APP.PixAiAccount>({ email: "", password: "", status: "" })
let curAccountIndex = -1

const showSettings = ref(false)

const handleEdit = (a: APP.PixAiAccount, i: number) => {
  if (disabled.value) {
    message.error("运行中不可以编辑账号信息")
  }

  editTitle.value = "编辑账号信息"
  showEditDelete.value = true
  curAccount.value = a
  curAccountIndex = i
  showEdit.value = true
}

const handleAdd = () => {
  editTitle.value = "添加账号"
  showEditDelete.value = false
  curAccount.value = { email: "", password: "", status: "" }
  curAccountIndex = -1
  showEdit.value = true
}

const postAccountChange = async () => {
  await tauri.invoke("set_accounts", {
    newAccounts: accounts.value,
  })

  // 更新 Job state
  tauri.invoke("get_job_state").then((job_state) => {
    store.state = job_state as APP.AppState
  })
}

const handleAccountSave = () => {
  if (curAccountIndex >= 0 && curAccountIndex < accounts.value.length && accounts.value.length > 0) {
    accounts.value[curAccountIndex] = curAccount.value
  } else {
    accounts.value.push(curAccount.value)
  }
  showEdit.value = false
  postAccountChange().then()
}

const handleAccountDelete = () => {
  if (curAccountIndex >= 0 && curAccountIndex < accounts.value.length && accounts.value.length > 0) {
    accounts.value.splice(curAccountIndex, 1)
  }
  showEdit.value = false
  postAccountChange().then()
}

const handleExit = async () => {
  await tauri.invoke("exit")
}

const handleStart = () => {
  tauri.invoke("start_claim").then(() => {
    message.info("开始签到~")
  })
}

const handleSettings = () => {
  showSettings.value = true
}

onMounted(() => {
  tauri.invoke("get_accounts").then((new_accounts) => {
    accounts.value = new_accounts as APP.PixAiAccount[]
  })

  listen<APP.AppState>("state_update", (e) => {
    store.state = e.payload
  })

  listen<number>("claim_finished", () => {
    message.success("签到成功！")
  })
})
</script>

<template>
  <div class="flex gap-2 absolute h-full w-full">
    <div class="grow relative">
      <div class="absolute top-0 left-0 w-full h-full flex flex-col gap-3">
        <div class="flex items-center gap-2">
          <n-button secondary @click="handleAdd" :disabled="disabled">
            <n-icon size="12"><Plus /></n-icon>&nbsp;添加账号</n-button
          >
          <div class="text-xs">点击账号可以修改或删除信息</div>
          <div class="grow" />
          <div>显示密码 <n-switch v-model:value="showPassword" /></div>
        </div>
        <div class="grow relative">
          <div class="absolute top-0 left-0 w-full h-full overflow-y-auto">
            <n-table :bordered="false" :single-line="false" class="" :disabled="disabled">
              <thead>
                <tr>
                  <th>账号</th>
                  <th>密码</th>
                  <th class="w-[20px]">状态</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="(a, i) in accounts" :key="i" @click="handleEdit(a, i)">
                  <td>{{ a.email }}</td>
                  <td>{{ showPassword ? a.password : "****" }}</td>
                  <td>
                    <account-status :status="store.state.account_status[i] ?? -1" />
                  </td>
                </tr>
              </tbody>
            </n-table>
            <div v-if="accounts.length == 0" class="p-5 bg-white">
              <n-empty></n-empty>
            </div>
          </div>
        </div>
      </div>
    </div>
    <div class="min-w-[200px] flex flex-col gap-2">
      <JobInfo />
      <div class="grow" />
      <n-button type="primary" @click="handleStart" :disabled="disabled">开始签到</n-button>
      <n-button strong secondary type="success" @click="handleSettings" :disabled="disabled">设置</n-button>
      <n-button strong secondary type="success" @click="handleExit">退出程序</n-button>
    </div>

    <n-modal v-model:show="showEdit" preset="dialog" :show-icon="false" :title="editTitle">
      <EditAccount
        v-model:value="curAccount"
        :show-delete="showEditDelete"
        @submit="handleAccountSave"
        @delete="handleAccountDelete"
      />
    </n-modal>
    <n-modal v-model:show="showSettings" preset="dialog" :show-icon="false" title="设置">
      <Settings />
    </n-modal>
  </div>
</template>

<style scoped></style>
