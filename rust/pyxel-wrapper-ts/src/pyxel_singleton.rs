use core::ptr::null_mut;
use engine::Pyxel;

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

pub fn pyxel() -> &'static mut Pyxel {
    unsafe {
        if PYXEL_INSTANCE.is_null() {
            panic!("Pyxel instance is not initialized");
        }
        &mut *PYXEL_INSTANCE
    }
}
