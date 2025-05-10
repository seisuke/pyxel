use pyxel_wrapper_ts_macros::tsmodule;

#[tsmodule(name = "pyxel")]
pub mod resouce_wrapper {

    use crate::pyxel::Image;
    use crate::pyxel_singleton::pyxel;
    use pyxel_wrapper_ts_macros::{tsclass, tsfunction, tsimpl};

    #[tsclass]
    #[allow(dead_code)]
    #[derive(Clone)]
    pub struct ImageList {
        inner: i32, //dummy
    }

    #[tsimpl]
    #[allow(dead_code)]
    impl ImageList {
        pub fn wrap(inner: i32) -> Self {
            Self { inner }
        }

        #[tsfunction]
        pub fn len(&self) -> usize {
            pyxel().images.lock().len()
        }

        #[tsfunction]
        pub fn get(&self, index: usize) -> Image {
            let inner = pyxel().images.lock()[index].clone();
            Image::wrap(inner)
        }
    }
}
