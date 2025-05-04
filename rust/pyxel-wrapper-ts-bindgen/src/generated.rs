// Auto-generated wrapper functions

use std::ffi::CStr;
use crate::pyxel::Image;
#[no_mangle]
pub extern "C" fn init(
    width: i32, 
    height: i32
) {
    let width = match width.try_into() { Ok(v) => v, Err(_) => return, };
let height = match height.try_into() { Ok(v) => v, Err(_) => return, };
    crate::pyxel::init(width, height)
}
#[no_mangle]
pub extern "C" fn cls(
    color: i32
) {
    let color = match color.try_into() { Ok(v) => v, Err(_) => return, };
    crate::pyxel::cls(color)
}
#[no_mangle]
pub extern "C" fn load(
    filename: *const u8, 
    excl_images: i32, 
    excl_tilemaps: i32, 
    excl_sounds: i32, 
    excl_musics: i32, 
    incl_colors: i32, 
    incl_channels: i32, 
    incl_tones: i32
) {
    let c_str = unsafe { CStr::from_ptr(filename as *const i8) };
let filename = match c_str.to_str() { Ok(s) => s, Err(_) => return, };
let excl_images = match excl_images { 0 => Some(false), 1 => Some(true), _ => None };
let excl_tilemaps = match excl_tilemaps { 0 => Some(false), 1 => Some(true), _ => None };
let excl_sounds = match excl_sounds { 0 => Some(false), 1 => Some(true), _ => None };
let excl_musics = match excl_musics { 0 => Some(false), 1 => Some(true), _ => None };
let incl_colors = match incl_colors { 0 => Some(false), 1 => Some(true), _ => None };
let incl_channels = match incl_channels { 0 => Some(false), 1 => Some(true), _ => None };
let incl_tones = match incl_tones { 0 => Some(false), 1 => Some(true), _ => None };
    crate::pyxel::load(filename, excl_images, excl_tilemaps, excl_sounds, excl_musics, incl_colors, incl_channels, incl_tones)
}
#[no_mangle]
pub extern "C" fn Image_new(
    width: i32, 
    height: i32
) -> *mut Image {
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
