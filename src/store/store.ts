import { reactive } from "vue"

type Store = {
  state: APP.AppState
}

export const store = reactive<Store>({
  state: { running: false, accounts_num: 0, concurrent: 2, jobs_left: 0, account_status: [] },
})
