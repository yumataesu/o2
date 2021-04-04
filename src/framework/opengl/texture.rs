use super::{Load, traits::{Allocate, New}};
use image::{GenericImage};
use super::utils;
use super::shader;
use super::vao;
#[derive(Debug, Default)]
pub struct Texture {
    id: gl::types::GLuint,
    width: i32,
    height: i32,
    quad_shader: shader::Shader,
    quad: vao::Vao,
    internal_format: gl::types::GLint,
    data_type: gl::types::GLuint,
    allocated: bool,
    min_filter: gl::types::GLuint,
	mag_filter: gl::types::GLuint,
	wrap_horizontal: gl::types::GLuint,
	wrap_vertical: gl::types::GLuint
}

/* 
https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexImage2D.xhtml

- internal_format
   GL_DEPTH_COMPONENT GL_DEPTH_STENCIL GL_RED GL_RG GL_RGB GL_RGBA 
*/

impl New<(i32, i32, i32)> for Texture {
    fn new(args: (i32, i32, i32)) -> Self {
        let width = args.0;
        let height = args.1;
        let internal_format = args.2;

        unsafe {
            let mut quad = vao::Vao::new();
            quad.create_quad();
            let mut quad_shader = shader::Shader::new();
            quad_shader.load((utils::VS_QUAD, utils::FS_QUAD));

            let mut id = std::mem::zeroed();
            gl::GenTextures(1, &mut id);

            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as f32);
            gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as f32);
            gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as f32);
            gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as f32);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, width, height, 0, gl::RGB, gl::UNSIGNED_BYTE, std::ptr::null());
            gl::BindTexture(gl::TEXTURE_2D, 0);

            Texture{ id: id, width:width, height:height, quad_shader: quad_shader, quad: quad, internal_format: gl::RGBA as i32, data_type: gl::UNSIGNED_BYTE, allocated: false, wrap_horizontal: gl::CLAMP_TO_EDGE, wrap_vertical: gl::CLAMP_TO_EDGE, min_filter: gl::LINEAR, mag_filter:  gl::LINEAR}
        }
    }
}



impl New<&str> for Texture {
    fn new(args: &str) -> Self {

        let path = args;
        let img = image::open(&std::path::Path::new(path)).expect("Failed to load image");
        
        let size = img.dimensions();
        let width = size.0 as i32;
        let height = size.1 as i32;
        let data = img.raw_pixels();

        unsafe {
            let mut quad = vao::Vao::new();
            quad.create_quad();
            let mut quad_shader = shader::Shader::new();
            quad_shader.load((utils::VS_QUAD, utils::FS_QUAD));

            let mut id = std::mem::zeroed();
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as f32);
            gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as f32);
            // set texture filtering parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, width, height, 0, gl::RGB, gl::UNSIGNED_BYTE, data.as_ptr() as *const _);
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, 0);

            Texture{ id: id, width: width, height: height, quad_shader: quad_shader, quad: quad, internal_format: gl::RGBA as i32, data_type: gl::UNSIGNED_BYTE, allocated: false, wrap_horizontal: gl::CLAMP_TO_EDGE, wrap_vertical: gl::CLAMP_TO_EDGE, min_filter: gl::LINEAR, mag_filter:  gl::LINEAR}
        }
    }
}


impl Allocate<(i32, i32, i32)> for Texture {
    fn allocate(&mut self, args: (i32, i32, i32)) -> &mut Self {
        self.width = args.0;
        self.height = args.1;
        self.internal_format = args.2;
        self.allocate()
    }
}


impl Texture {

    fn load_image(&mut self, path: &str) -> &mut Self {
        let img = image::open(&std::path::Path::new(path)).expect("Failed to load image");
        
        let size = img.dimensions();
        self.width = size.0 as i32;
        self.height = size.1 as i32;
        let data = img.raw_pixels();
        
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, self.wrap_horizontal as f32);
            gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, self.wrap_vertical as f32);
            // set texture filtering parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, self.min_filter as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, self.mag_filter as i32);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, self.width, self.height, 0, gl::RGB, gl::UNSIGNED_BYTE, data.as_ptr() as *const _);
            gl::GenerateMipmap(gl::TEXTURE_2D);
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
            gl::BindTexture(gl::TEXTURE_2D, 0);
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
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        self
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }


    pub fn get_width(&self) -> i32 {
        self.width
    }


    pub fn get_height(&self) -> i32 {
        self.height
    }

    
    pub fn get(&self) -> &gl::types::GLuint {
        &self.id
    }

    pub fn draw(&self) {
        unsafe {
            self.quad_shader.begin();
            self.quad_shader.uniform_texture("u_src", self.get());
            self.quad.draw_elements(gl::TRIANGLES);
            self.quad_shader.end();
        }
    }


    fn allocate(&mut self)-> &mut Self {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, self.min_filter as f32);
            gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, self.min_filter as f32);
            gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, self.wrap_horizontal as f32);
            gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, self.wrap_vertical as f32);
            gl::TexImage2D(gl::TEXTURE_2D, 0, self.internal_format, self.width, self.height, 0, self.internal_format as u32, self.data_type, std::ptr::null());
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        self
    }
}
