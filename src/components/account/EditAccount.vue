<script setup lang="ts">
import { useDialog } from "naive-ui"
import { reactive, ref } from "vue"
const account = defineModel<APP.PixAiAccount>("value", { required: true })

withDefaults(defineProps<{ showDelete?: boolean }>(), { showDelete: false })

const emits = defineEmits<{
  (e: "submit"): void
  (e: "delete"): void
}>()

const formRef = ref()

const rules = reactive({
  email: {
    required: true,
    message: "请输入邮箱",
    trigger: ["blur"],
  },
  password: {
    required: true,
    message: "请输入密码",
    trigger: ["blur"],
  },
})

const handleSubmit = (e: MouseEvent) => {
  e.preventDefault()
  formRef.value?.validate((errors) => {
    if (!errors) {
      emits("submit")
    } else {
    }
  })
}

const dialog = useDialog()

const handleDelete = () => {
  dialog.warning({
    title: "警告",
    content: `确定要删除账号 ${account.value.email} 吗`,
    positiveText: "确定",
    negativeText: "取消",
    onPositiveClick: () => {
      emits("delete")
    },
  })
}
</script>

<template>
  <n-form ref="formRef" :label-width="80" :model="account" size="medium" :rules="rules">
    <n-form-item label="邮箱" path="email">
      <n-input v-model:value="account!.email" placeholder="输入您的邮箱" />
    </n-form-item>
    <n-form-item label="密码" path="password">
      <n-input type="password" v-model:value="account!.password" placeholder="输入您的密码" />
    </n-form-item>

    <div class="flex flex-row-reverse gap-2">
      <n-button attr-type="button" @click="handleSubmit" type="primary"> 保存 </n-button>
      <n-button v-if="showDelete" attr-type="button" @click="handleDelete" type="error"> 删除 </n-button>
    </div>
  </n-form>
</template>

<style scoped></style>
