use pyxel_wrapper_ts_macros::tsmodule;

mod image_wrapper;
mod pyxel_singleton;

#[tsmodule]
pub mod pyxel {
    pub use crate::image_wrapper::image_wrapper::Image;
    use crate::pyxel_singleton::set_pyxel_instance;

    pub fn init(width: i32, height: i32) {
        let width = match width.try_into() {
            Ok(w) => w,
            Err(_) => return,
        };
        let height = match height.try_into() {
            Ok(h) => h,
            Err(_) => return,
        };

        let pyxel = pyxel::init(width, height, None, None, None, None, None, None);
        set_pyxel_instance(pyxel);
    }

    pub fn cls(color: i32) {
        match (color.try_into(), crate::pyxel_singleton::pyxel()) {
            (Ok(color), Some(pyxel)) => {
                pyxel.cls(color);
                pyxel.circb(30.0, 30.0, 20.0, 7);
                pyxel.show();
            }
            _ => {}
        }
    }
}
