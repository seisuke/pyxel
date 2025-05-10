use pyxel_wrapper_ts_macros::tsmodule;

#[tsmodule(name = "pyxel")]
pub mod image_wrapper {
    use pyxel_wrapper_ts_macros::{tsclass, tsfunction, tsimpl};

    #[tsclass]
    #[allow(dead_code)]
    #[derive(Clone)]
    pub struct Image {
        pub(crate) inner: engine::SharedImage,
    }

    #[tsimpl]
    #[allow(dead_code)]
    impl Image {
        pub fn wrap(inner: engine::SharedImage) -> Self {
            Self { inner }
        }

        #[tsfunction]
        pub fn new(width: u32, height: u32) -> Self {
            Self::wrap(engine::Image::new(width, height))
        }

        #[tsfunction]
        pub fn width(&self) -> u32 {
            self.inner.lock().width()
        }

        #[tsfunction]
        pub fn height(&self) -> u32 {
            self.inner.lock().height()
        }
    }
}
