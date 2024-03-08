use std::collections::VecDeque;
use std::ops::Sub;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, SystemTime};

use parking_lot::Mutex;
use tauri::{async_runtime, State, Window};
use thirtyfour::prelude::*;

pub use chrome_driver::stop_chrome_driver;

use crate::claimer::chrome_driver::guard_driver_running;
use crate::dto::{JobState, PixAiAccount};
use crate::state::AppState;

mod chrome_driver;

async fn claim_base(account: PixAiAccount, port: u16) -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new(&format!("http://localhost:{port}"), caps).await?;

    // 登录
    {
        driver.goto("https://pixai.art/login").await?;
        let use_email_btn = driver.query(By::Tag("button")).with_text("Log in with email").first().await?;
        use_email_btn.click().await?;

        let email_field = driver.find(By::Id("email-input")).await?;
        let password_field = driver.find(By::Id("password-input")).await?;
        email_field.send_keys(&account.email).await?;
        password_field.send_keys(&account.password).await?;

        let submit_btn = driver.find(By::Css("button[type='submit']")).await?;
        submit_btn.click().await?;
    }

    // 前往签到页面
    {
        // 等待页面加载
        while driver.find_all(By::Tag("header")).await?.is_empty() {
            tokio::time::sleep(Duration::from_secs(1)).await;
        }

        // 清理弹窗
        driver.execute(r#"for (let item of document.getElementsByClassName("MuiModal-root")) { item.remove(); }"#, Vec::new()).await?;

        // 前往页面
        let header_elem = driver.find(By::Tag("header")).await?;
        let avatar_btn = header_elem.query(By::Tag("div")).all().await?;
        avatar_btn.last().unwrap().click().await?;
        let profile_btn = driver.query(By::ClassName("MuiListItemText-primary")).with_text("Profile").first().await?;
        profile_btn.click().await?;
    }

    // 点击签到按钮
    {
        // 等待页面加载
        while driver.find_all(By::Css("span[class='contents']")).await?.is_empty() {
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
        tokio::time::sleep(Duration::from_secs(1)).await;

        // 清理弹窗
        driver.execute(r#"for (let item of document.getElementsByClassName("MuiModal-root")) { item.remove(); }"#, Vec::new()).await?;

        // 获取登录按钮
        let ret = driver.execute(r#"
            for (let item of document.getElementsByClassName("bg-black")) {
                    if (item.innerText.startsWith("Claim")) {
                        return item;
                    }
            }
        "#, Vec::new()).await?;
        let claim_btn = ret.element()?;
        if claim_btn.text().await? == "Claimed" {
            // 已经签到过了
            driver.quit().await?;
            return Ok(());
        }

        // 签到
        claim_btn.click().await?;

        // 等待签到成功
        while driver.find_all(By::ClassName("Toastify")).await?.is_empty() {
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }

    // Always explicitly close the browser.
    driver.quit().await?;

    Ok(())
}

pub fn claim(account: PixAiAccount) -> bool {
    let port = guard_driver_running();
    async_runtime::block_on(async move {
        claim_base(account, port).await.is_ok()
    })
}

/// 测试用
#[cfg(test)]
pub async fn claim_async(account: PixAiAccount) -> bool {
    let port = guard_driver_running();
    claim_base(account, port).await.is_ok()
}

fn run_claim<'a>(accounts: Arc<Vec<PixAiAccount>>, job_state_lock: Arc<Mutex<JobState>>, window: Window) {
    // 初始化Job state
    let account_status = vec![0; accounts.len()];
    let concurrent_num;
    {
        let mut job_state = job_state_lock.lock();
        job_state.jobs_left = accounts.len() as u32;
        concurrent_num = job_state.concurrent;
        job_state.running = true;
        job_state.account_status = account_status.clone();
        window.emit("state_update", job_state.clone()).expect("Emit event 'state_update' failed");
    }
    let account_status = Arc::new(Mutex::new(account_status));

    // 准备回调函数
    let last_finish_time = Arc::new(Mutex::new(SystemTime::now()));
    let on_start = {
        let account_status = account_status.clone();
        let job_state_lock = job_state_lock.clone();
        let window = window.clone();
        Arc::new(move |index: usize| {
            let as_owned;
            {
                let mut asl = account_status.lock();
                asl[index] = 1;
                as_owned = asl.clone();
            }
            let mut job_state = job_state_lock.lock();
            job_state.account_status = as_owned;
            window.emit("state_update", job_state.clone()).expect("Emit event 'state_update' failed");
        })
    };
    let on_finish = {
        let account_status = account_status.clone();
        let job_state_lock = job_state_lock.clone();
        let window = window.clone();
        Arc::new(move |index: usize, success: bool| {
            let as_owned;
            {
                let mut asl = account_status.lock();
                asl[index] = if success { 2 } else { 3 };
                as_owned = asl.clone();
            }

            let time_left;
            {
                let mut lft_lock = last_finish_time.lock();
                let last_duration = SystemTime::now().duration_since(lft_lock.clone());
                *lft_lock = SystemTime::now();
                time_left = last_duration.unwrap().as_secs();
            }

            let mut job_state = job_state_lock.lock();
            job_state.jobs_left -= 1;
            job_state.account_status = as_owned;
            job_state.time_left = Some(time_left * job_state.jobs_left as u64);
            window.emit("state_update", job_state.clone()).expect("Emit event 'state_update' failed");
        })
    };

    // 开始任务
    let next_job = Arc::new(AtomicUsize::new(0));
    let mut handle_set = VecDeque::new();
    for _ in 0..concurrent_num {
        let accounts = accounts.clone();
        let next_job = next_job.clone();
        let on_start = on_start.clone();
        let on_finish = on_finish.clone();

        let handle = thread::spawn(move || {
            loop {
                let job = next_job.fetch_add(1, Ordering::SeqCst);
                if job >= accounts.len() {
                    return;
                }

                let account = accounts[job].clone();
                on_start(job);
                let suc = claim(account);
                on_finish(job, suc);
            }
        });
        handle_set.push_front(handle);
    }
    while let Some(handle) = handle_set.pop_back() {
        let _ = handle.join();
    }

    // 结束
    let mut job_state = job_state_lock.lock();
    job_state.running = false;
    window.emit("state_update", job_state.clone()).expect("Emit event 'state_update' failed");
    window.emit("claim_finished", 0).expect("Emit event 'claim_finished' failed");
}

#[tauri::command]
pub fn start_claim(app_state: State<AppState>, window: Window) {
    let accounts = Arc::new(app_state.accounts.read().clone());
    let job_state_lock = app_state.job_state.clone();
    thread::spawn(move || {
        run_claim(accounts, job_state_lock, window);
    });
}

