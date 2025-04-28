use once_cell::sync::Lazy;
use std::sync::Mutex;

/// Pyxel本体を持つ構造体
pub struct Pyxel {
    width: i32,
    height: i32,
}

impl Pyxel {
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }

    pub fn update(&mut self) {
        // 今は何もしない（必要なら後で追加）
    }

    pub fn draw(&self) {
        // 今は何もしない（必要なら後で追加）
    }

    pub fn cls(&self, _color: i32) {
        // 今は何もしない（色を受け取るだけ）
    }
}

/// グローバルシングルトン

static mut SINGLETON: Lazy<Mutex<Option<Pyxel>>> = Lazy::new(|| Mutex::new(None));

/// Pyxelインスタンスを初期化する
pub fn init(width: i32, height: i32) {
    let mut singleton = unsafe { SINGLETON.lock().unwrap() };
    *singleton = Some(Pyxel::new(width, height));
}

/// Pyxelインスタンスを借用する（参照）
pub fn with_pyxel<F, R>(f: F) -> R
where
    F: FnOnce(&Pyxel) -> R,
{
    let singleton = unsafe { SINGLETON.lock().unwrap() };
    f(singleton.as_ref().expect("Pyxel is not initialized"))
}

/// Pyxelインスタンスを借用する（可変参照）
pub fn with_pyxel_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut Pyxel) -> R,
{
    let mut singleton = unsafe { SINGLETON.lock().unwrap() };
    f(singleton.as_mut().expect("Pyxel is not initialized"))
}
