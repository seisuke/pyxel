use core::ptr::null_mut;
use pyxel::Pyxel;

static mut PYXEL_INSTANCE: *mut Pyxel = null_mut();

pub fn set_pyxel_instance(pyxel: Pyxel) {
    unsafe {
        PYXEL_INSTANCE = Box::into_raw(Box::new(pyxel));
    }
}

pub fn pyxel() -> Option<&'static mut Pyxel> {
    unsafe {
        if PYXEL_INSTANCE.is_null() {
            None
        } else {
            Some(&mut *PYXEL_INSTANCE)
        }
    }
}
