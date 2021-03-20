use super::traits::{Allocate, Update};
use image::{GenericImage};

const VS_SRC: &'static [u8] = b"
#version 450

layout (location = 0) in vec3 position;
layout (location = 2) in vec2 texcoord;

out vec2 v_texcoord;

void main() {
    gl_Position = vec4(position, 1.0);
    v_texcoord = texcoord;
}
\0";

const FS_SRC: &'static [u8] = b"
#version 450
uniform sampler2D u_src;

in vec2 v_texcoord;

layout (location = 0) out vec4 FragColor;

void main() {
    vec4 result = texture(u_src, v_texcoord);
    FragColor = result;
}
\0";

#[derive(Debug, Default)]
pub struct Texture {
    id: gl::types::GLuint,
    width: i32,
    height: i32,
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

impl Allocate<(i32, i32, i32)> for Texture {
    fn allocate(&mut self, args: (i32, i32, i32)) -> &mut Self {
        unsafe {
            self.width = args.0;
            self.height = args.1;
            self.internal_format = args.2;
            self.allocate()
        }
    }
}


impl Texture {
    pub fn new() -> Texture {
        unsafe {
            let mut id = std::mem::zeroed();
            gl::GenTextures(1, &mut id);
            Texture{ id: id, width:0, height:0, internal_format: gl::RGBA as i32, data_type: gl::UNSIGNED_BYTE, allocated: false, wrap_horizontal: gl::CLAMP_TO_EDGE, wrap_vertical: gl::CLAMP_TO_EDGE, min_filter: gl::LINEAR, mag_filter:  gl::LINEAR}
        }
    }


    pub fn load_image(&mut self, path: &str) -> &mut Self {
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
