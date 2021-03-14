use super::texture;
// use super::traits::{Allocate, Update};
#[derive(Debug, Default)]
pub struct FrameBuffer {
    id: gl::types::GLuint,
    width: i32,
    height: i32,
}

impl FrameBuffer {
    fn new() -> FrameBuffer {
        // let mut id;
        unsafe {
            let mut id = std::mem::zeroed();
            gl::GenFramebuffers(1, &mut id);
            FrameBuffer { id: id, width: 512, height:512 }
        }
    }

    pub fn allocate(&mut self, texture: texture::Texture, attach_point: gl::types::GLenum) -> &mut Self {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
            //texture.bind();
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, attach_point, gl::TEXTURE_2D, *texture.get(), 0);
        }

        self
    }

    pub fn clear(&self) {
        unsafe {

        }
    }

    pub fn begin(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
        }
    }

    pub fn end(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    pub fn draw(&self) {

    }

    pub fn get(&self) -> &gl::types::GLuint {
        &self.id
    }
}