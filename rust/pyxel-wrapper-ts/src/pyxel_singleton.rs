use core::ptr::null_mut;
use pyxel::Pyxel;

static mut PYXEL_INSTANCE: *mut Pyxel = null_mut();

pub fn set_pyxel_instance(pyxel: Pyxel) {
    unsafe {
        PYXEL_INSTANCE = Box::into_raw(Box::new(pyxel));
    }
}

pub fn with_pyxel<F, R>(f: F) -> Option<R>
where
    F: FnOnce(&mut Pyxel) -> R,
{
    unsafe {
        if PYXEL_INSTANCE.is_null() {
            None
        } else {
            Some(f(&mut *PYXEL_INSTANCE))
        }
    }
}
