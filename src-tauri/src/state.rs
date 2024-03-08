use std::fs::File;
use std::{fs, io, thread};
use std::io::Read;
use std::sync::Arc;

use parking_lot::{Mutex, RwLock};
use platform_dirs::AppDirs;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::{APP_NAME, state};
use crate::dto::{JobState, PixAiAccount};
use crate::error::{AppError, AppResult};

#[derive(Debug, Clone)]
pub struct AppState {
    pub accounts: Arc<RwLock<Vec<PixAiAccount>>>,
    pub job_state: Arc<Mutex<JobState>>,
}

#[derive(Deserialize, Serialize)]
pub struct AppStateUnlocked {
    accounts: Vec<PixAiAccount>,
}

impl Default for AppStateUnlocked {
    fn default() -> Self {
        AppStateUnlocked {
            accounts: vec![],
        }
    }
}

impl From<AppState> for AppStateUnlocked {
    fn from(value: AppState) -> Self {
        let accounts_lock = value.accounts.clone();
        let accounts_guard = accounts_lock.read();
        Self {
            accounts: accounts_guard.clone(),
        }
    }
}

impl From<AppStateUnlocked> for AppState {
    fn from(value: AppStateUnlocked) -> Self {
        Self {
            accounts: Arc::new(RwLock::new(value.accounts)),
            job_state: Arc::new(Mutex::new(JobState {
                running: false,
                accounts_num: 0,
                jobs_left: 0,
                concurrent: 1,
                time_left: None,
                account_status: vec![],
            },)),
        }
    }
}

pub fn load_app_state() -> AppResult<AppStateUnlocked> {
    let app_dirs = AppDirs::new(Some(APP_NAME), false).unwrap();
    let mut config_dir = app_dirs.config_dir;
    config_dir.push("config");

    let conf_file = File::open(&config_dir);
    match conf_file {
        Err(ref err) if err.kind() == io::ErrorKind::NotFound => Ok(AppStateUnlocked::default()),
        Ok(mut file) => Ok(serde_json::from_reader(&mut file).unwrap_or_default()),
        Err(err) => Err(err.into()),
    }
}

pub fn save_app_state(app_state: impl Into<AppStateUnlocked>) -> AppResult<()> {
    let app_state: AppStateUnlocked = app_state.into();
    let app_dirs = AppDirs::new(Some(APP_NAME), false).unwrap();
    let mut config_dir = app_dirs.config_dir;
    fs::create_dir_all(&config_dir)?;

    config_dir.push("config");
    let conf_file = File::create(&config_dir);
    match conf_file {
        Ok(mut file) => {
            let _ = serde_json::to_writer(&mut file, &app_state);
            Ok(())
        }
        Err(err) => Err(err.into()),
    }
}

#[tauri::command]
pub fn get_accounts(app_state: State<AppState>) -> Vec<PixAiAccount> {
    let lock = app_state.accounts.clone();
    let guard = lock.read();
    guard.clone()
}

#[tauri::command]
pub fn set_accounts(new_accounts: Vec<PixAiAccount>, app_state: State<AppState>) -> AppResult<()> {
    let mut state = app_state.job_state.lock();
    if state.running {
        Err(AppError::GeneralError("不可在签到时改变账户信息".to_owned()))
    } else {
        state.accounts_num = new_accounts.len() as u32;
        state.jobs_left = new_accounts.len() as u32;
        drop(state);

        let mut accounts = app_state.accounts.write();
        *accounts = new_accounts;
        drop(accounts);

        let cloned_app_state = AppState::clone(&app_state);
        thread::spawn(move || {
            save_app_state(cloned_app_state).unwrap();
        });

        Ok(())
    }
}

#[tauri::command]
pub fn get_job_state(app_state: State<AppState>) -> JobState {
    let lock = app_state.job_state.clone();
    let guard = lock.lock();
    guard.clone()
}
