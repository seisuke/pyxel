use pyxel_wrapper_ts_macros::tsmodule;

#[tsmodule(name = "pyxel")]
pub mod image_wrapper {
    use pyxel_wrapper_ts_macros::{tsclass, tsfunction, tsimpl};

    #[tsclass]
    #[allow(dead_code)]
    pub struct Image {
        width: i32,
        height: i32,
    }

    #[tsimpl]
    #[allow(dead_code)]
    impl Image {
        #[tsfunction]
        pub fn new(width: i32, height: i32) -> Self {
            Self { width, height }
        }

        #[tsfunction]
        pub fn width(&self) -> i32 {
            self.width
        }

        #[tsfunction]
        pub fn height(&self) -> i32 {
            self.height
        }
    }
}
