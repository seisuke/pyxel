// Auto-generated wrapper functions

#[no_mangle]
pub extern "C" fn init(_width: i32, _height: i32) {
    crate::pyxel::init(_width, _height)
}

#[no_mangle]
pub extern "C" fn Image_new(width: i32, height: i32) {
    crate::pyxel::Image::new(width, height)
}

#[no_mangle]
pub extern "C" fn Image_width() {
    crate::pyxel::Image::width()
}

#[no_mangle]
pub extern "C" fn Image_height() {
    crate::pyxel::Image::height()
}

