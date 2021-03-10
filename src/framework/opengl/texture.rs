
use super::traits::{Allocate, Update};
use image::io::Reader as ImageReader;

use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};

#[derive(Debug, Default)]
pub struct Texture {
    id: gl::types::GLuint,
    width: i32,
    height: i32,
    internal_format: gl::types::GLuint,
    allocated: bool,
    min_filter: gl::types::GLint,
	mag_filter: gl::types::GLint,
	wrapModeHorizontal: gl::types::GLint,
	wrapModeVertical: gl::types::GLint
}

// impl Allocate<(width: u32, height: u32)> for Texture {

// }

impl Texture {
    // pub fn new() {
    //     Texture{ default() }
    // }

    pub fn type_of<T>(&mut self, _: T) -> String {
        let a = std::any::type_name::<T>();
        return a.to_string();
    }

    pub fn allocate(&mut self, width: u32, height: u32) {

        // println!("dimensions {:?}", img.dimensions());
        //image_into_bytes
        //let p = GenericImage::image_as_bytes(img);
        unsafe {
            gl::GenTextures(1, &mut self.id);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, self.width, self.height, 0, gl::RGBA, gl::UNSIGNED_BYTE, std::ptr::null());  // init to black...
            gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as f32);
            gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as f32);
            gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as f32);
            gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as f32);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn load(&mut self) {
        let img = image::open("data/te.jpg").unwrap();
        //img.as_bytes()
        //img.pixels();
        // println!("{:p}", self.type_of(img.as_bytes()[0]));
        //println!("{:p}", &img.as_bytes()[0]);

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexSubImage2D(gl::TEXTURE_2D, 0, 0, 0, self.width, self.height, gl::RGBA, gl::UNSIGNED_BYTE, img.as_bytes()[0] as *const _);
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }


    pub fn get(&self) -> &gl::types::GLuint {
        &self.id
    }
}
