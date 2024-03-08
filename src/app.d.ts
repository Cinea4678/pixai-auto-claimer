namespace APP {
  type PixAiAccount = {
    email: string
    password: string
    status: string
  }

  type AppState = {
    running: boolean
    accounts_num: number
    jobs_left: number
    concurrent: number
    time_left?: number
    account_status: number[]
  }

  type Settings = {
    concurrent: number
  }
}
