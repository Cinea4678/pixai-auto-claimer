// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, State, Window};

pub use state::{get_accounts, get_job_state, set_accounts};
use crate::claimer::start_claim;
use crate::dto::Settings;

use crate::state::AppState;

pub mod claimer;
pub mod dto;
pub mod error;
pub mod state;
pub mod panic;

pub static APP_NAME: &'static str = "pixai-auto-claimer";

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn exit(handle: AppHandle) {
    handle.exit(0);
}

#[tauri::command]
fn set_settings(app_state: State<AppState>, settings: Settings, window: Window) {
    let mut job_state = app_state.job_state.lock();
    if job_state.running {
        return;
    }
    job_state.concurrent = settings.concurrent;
    window.emit("state_update", job_state.clone()).expect("Emit event 'state_update' failed");
}

pub struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        claimer::stop_chrome_driver();
    }
}

fn main() {
    let cleanup = CleanUp;
    std::panic::set_hook(Box::new(panic::panic_handler));

    let app_state: AppState = state::load_app_state().unwrap().into();

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![exit, get_job_state, get_accounts, set_accounts, start_claim, set_settings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
