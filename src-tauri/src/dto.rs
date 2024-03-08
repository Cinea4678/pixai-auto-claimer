use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PixAiAccount {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobState {
    pub running: bool,
    pub accounts_num: u32,
    pub jobs_left: u32,
    pub concurrent: u32,
    pub time_left: Option<u64>,
    pub account_status: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub concurrent: u32,
}


