use pyxel_wrapper_ts_macros::tsmodule;

mod image_wrapper;
mod pyxel_singleton;

#[tsmodule]
pub mod pyxel {
    pub use crate::image_wrapper::image_wrapper::Image;
    use crate::pyxel_singleton::{init as init_singleton, with_pyxel_mut};

    pub fn init(width: i32, height: i32) {
        init_singleton(width, height);
    }

    pub fn cls(color: i32) {
        with_pyxel_mut(|pyxel| pyxel.cls(color.try_into().unwrap()));
    }
}
