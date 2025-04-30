use once_cell::sync::Lazy;
use pyxel::{init as pyxel_init, Pyxel};
use std::sync::Mutex;

static SINGLETON: Lazy<Mutex<Option<Pyxel>>> = Lazy::new(|| Mutex::new(None));

pub fn init(width: i32, height: i32) {
    let mut singleton = SINGLETON.lock().unwrap();
    *singleton = Some(pyxel_init(
        width.try_into().unwrap(),
        height.try_into().unwrap(),
        None,
        None,
        None,
        None,
        None,
        None,
    ));
}

/// Pyxelインスタンスを借用する（参照）
#[allow(dead_code)]
pub fn with_pyxel<F, R>(f: F) -> R
where
    F: FnOnce(&Pyxel) -> R,
{
    let singleton = SINGLETON.lock().unwrap();
    f(singleton.as_ref().expect("Pyxel is not initialized"))
}

/// Pyxelインスタンスを借用する（可変参照）
pub fn with_pyxel_mut<F, R>(f: F) -> R
where
    F: FnOnce(&mut Pyxel) -> R,
{
    let mut singleton = SINGLETON.lock().unwrap();
    f(singleton.as_mut().expect("Pyxel is not initialized"))
}
