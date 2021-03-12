
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
    min_filter: gl::types::GLuint,
	mag_filter: gl::types::GLuint,
	wrap_horizontal: gl::types::GLuint,
	wrap_vertical: gl::types::GLuint
}

// impl Allocate<(width: u32, height: u32)> for Texture {

// }

impl Texture {
    pub fn new() -> Texture {
        unsafe {
            let mut id = std::mem::zeroed();
            gl::GenTextures(1, &mut id);
            Texture{ id: id, width:0, height:0, wrap_horizontal: gl::CLAMP_TO_EDGE, wrap_vertical: gl::CLAMP_TO_EDGE, min_filter: gl::LINEAR, mag_filter:  gl::LINEAR}
        }
    }


    pub fn load_image(&mut self, path: &str) -> &mut Self {
        let img = image::open(path).expect("Failed to load image");
        let (w, h) = img.dimensions();
        let data = img.raw_pixels();
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as f32);
            gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as f32);
            gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as f32);
            gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as f32);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, w as i32, h as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, &data[0] as *const u8 as *const _);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        self
    }

    pub fn set_wrap_mode(&mut self, horizontal: gl::types::GLuint, vertical: gl::types::GLuint) -> &mut Self {
        unsafe {
            self.wrap_horizontal = horizontal;
            self.wrap_vertical = vertical;
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, self.wrap_horizontal as f32);
            gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, self.wrap_vertical as f32);
        }
        self
    }

    pub fn set_filter_mode(&mut self, min_filter: gl::types::GLuint, mag_filter: gl::types::GLuint) -> &mut Self {
        unsafe {
            self.min_filter = min_filter;
            self.mag_filter = mag_filter;
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, self.min_filter as f32);
            gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, self.mag_filter as f32);
        }
        self
    }


    pub fn get(&self) -> &gl::types::GLuint {
        &self.id
    }
}
