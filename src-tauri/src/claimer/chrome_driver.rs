use std::path::PathBuf;
use std::process;

use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use which::which;

static CHROME_DRIVER_PATH: OnceCell<PathBuf> = OnceCell::new();
static CHROME_DRIVER_PROCESS: OnceCell<Mutex<Option<(u16, process::Child)>>> = OnceCell::new();

pub fn find_chrome_driver() -> Option<PathBuf> {
    which("chromedriver").ok()
}

/// 本方法在程序正常或异常终止时清理ChromeDriver进程，本方法不可panic。
pub fn stop_chrome_driver() {
    if let Some(p) = CHROME_DRIVER_PROCESS.get() {
        if let Some(mut guard) = p.try_lock() {
            if guard.is_some() {
                let proc = guard.as_mut().unwrap();
                let _ = proc.1.kill();
            }
        }
    }
}

/// 启动浏览器driver
fn start_chrome_driver() -> (u16, process::Child) {
    let rand_port = portpicker::pick_unused_port().expect("No free ports left!");
    let driver = CHROME_DRIVER_PATH.get().unwrap().clone();
    let proc = process::Command::new(driver.into_os_string())
        .arg(format!("--port={rand_port}"))
        .spawn()
        .expect("failed to start chrome driver");
    (rand_port, proc)
}

/// 确保浏览器driver正在正常运行，返回端口号
pub fn guard_driver_running() -> u16 {
    // 假定此处已经确认了用户的电脑上有chrome driver
    CHROME_DRIVER_PATH.get_or_init(|| find_chrome_driver().unwrap());
    let mut driver_process = CHROME_DRIVER_PROCESS.get_or_init(|| Mutex::new(Some(start_chrome_driver()))).lock();
    if driver_process.is_none() {
        *driver_process = Some(start_chrome_driver());
    }

    // 确认进程没有退出
    let exited;
    {
        let p_ref = driver_process.as_mut().unwrap();
        let wait_res = p_ref.1.try_wait().expect("error attempting to wait chrome_driver.");
        exited = wait_res.is_some();
    }

    // 若进程退出了，那么需要重新启动
    if exited {
        *driver_process = Some(start_chrome_driver());
    }

    driver_process.as_ref().unwrap().0
}
