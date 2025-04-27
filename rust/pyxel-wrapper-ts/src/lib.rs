mod generated;

use pyxel_wrapper_ts_macros::tsfunction;


extern "C" {
    fn console_log(ptr: *const u8, len: usize);
}

pub fn log(s: &str) {
    unsafe {
        console_log(s.as_ptr(), s.len());
    }
}

// マクロはformatを使わず、直接文字列を渡すだけ！
macro_rules! console_log {
    ($msg:expr) => {
        log($msg)
    };
}

#[tsfunction]
pub fn init(width: i32, height: i32) {
    // フォーマットしたいなら固定メッセージだけでOK
    log("init called");
}

#[tsfunction]
pub fn update() {
    log("update called");
}

#[tsfunction]
pub fn draw() {
    log("draw called - drawing graphics");
}

