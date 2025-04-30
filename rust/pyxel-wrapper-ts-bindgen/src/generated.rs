// Auto-generated wrapper functions

use crate::pyxel::Image;

#[no_mangle]
pub extern "C" fn init(width: i32, height: i32) {
    crate::pyxel::init(width, height)
}

#[no_mangle]
pub extern "C" fn cls(color: i32) {
    crate::pyxel::cls(color)
}

#[no_mangle]
pub extern "C" fn Image_new(width: i32, height: i32) -> *mut Image {
    Box::into_raw(Box::new(crate::pyxel::Image::new(width, height)))
}

#[no_mangle]
pub extern "C" fn Image_width(ptr: *const Image) -> i32 {
    unsafe { &*ptr }.width()
}

#[no_mangle]
pub extern "C" fn Image_height(ptr: *const Image) -> i32 {
    unsafe { &*ptr }.height()
}

