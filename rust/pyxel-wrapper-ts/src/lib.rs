use pyxel_wrapper_ts_macros::tsmodule;

mod image_wrapper;

#[tsmodule]
pub mod pyxel {

    use pyxel_wrapper_ts_macros::tsfunction;
    #[tsfunction]
    pub fn init(_width: i32, _height: i32) {
        // 省略
    }
}
