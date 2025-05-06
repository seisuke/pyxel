use pyxel_wrapper_ts_macros::tsmodule;

mod image_wrapper;
mod pyxel_singleton;

#[tsmodule]
pub mod pyxel {
    pub use crate::image_wrapper::image_wrapper::Image;
    use crate::pyxel_singleton::{set_pyxel_instance, with_pyxel};
    use pyxel_wrapper_ts_macros::tsfunction;

    #[tsfunction(body = "await ready;")]
    pub fn init(width: u32, height: u32) {
        let pyxel = pyxel::init(width, height, None, None, None, None, None, None);
        set_pyxel_instance(pyxel);
    }

    #[tsfunction]
    pub fn cls(color: pyxel::Color) {
        with_pyxel(|pyxel| {
            pyxel.cls(color);
            pyxel.circb(30.0, 30.0, 20.0, 7);
            pyxel.show();
        });
    }

    #[tsfunction(body = "await fetchAndLoadResource(filename);")]
    pub fn load(
        filename: &str,
        excl_images: Option<bool>,
        excl_tilemaps: Option<bool>,
        excl_sounds: Option<bool>,
        excl_musics: Option<bool>,
        incl_colors: Option<bool>,
        incl_channels: Option<bool>,
        incl_tones: Option<bool>,
    ) {
        with_pyxel(|pyxel| {
            pyxel.load(
                filename,
                excl_images,
                excl_tilemaps,
                excl_sounds,
                excl_musics,
                incl_colors,
                incl_channels,
                incl_tones,
            );
        });
    }
}
