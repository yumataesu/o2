
extern crate image;
use super::traits::{Allocate, Update};
use image::{GenericImage};

#[derive(Debug, Default)]
pub struct Texture {
    id: gl::types::GLuint,
    width: i32,
    height: i32,
    // internal_format: gl::types::GLuint,
    // allocated: bool,
    // min_filter: gl::types::GLint,
	// mag_filter: gl::types::GLint,
	// wrapModeHorizontal: gl::types::GLint,
	// wrapModeVertical: gl::types::GLint
}

// impl Allocate<(width: u32, height: u32)> for Texture {

// }

impl Texture {
    pub fn new() -> Texture {
        unsafe {
            let mut id = std::mem::zeroed();
            gl::GenTextures(1, &mut id);
            Texture{ id: id, width:0, height:0 }
        }
    }


    pub fn load(&mut self, path: &str) {
        let img = image::open(path).expect("Failed to load texture");
        let (w, h) = img.dimensions();
        let data = img.raw_pixels();
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as f32);
            gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as f32);
            gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as f32);
            gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as f32);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, w as i32, h as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, std::ptr::null());
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, w as i32, h as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, &data[0] as *const u8 as *const _);
        }
    }


    pub fn get(&self) -> &gl::types::GLuint {
        &self.id
    }
}
