use super::texture;
// use super::traits::{Allocate, Update};
#[derive(Debug, Default)]
pub struct FrameBuffer {
    id: gl::types::GLuint,
    width: i32,
    height: i32,
}

impl FrameBuffer {
    pub fn new() -> FrameBuffer {
        unsafe {
            let mut id = std::mem::zeroed();
            gl::GenFramebuffers(1, &mut id);
            FrameBuffer { id: id, width: 512, height:512 }
        }
    }

    pub fn allocate(&mut self, texture: texture::Texture, attach_point: gl::types::GLenum) -> &mut Self {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
            texture.bind();
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, attach_point, gl::TEXTURE_2D, *texture.get(), 0);
            texture.unbind();

            let mut rbo = 0;
            gl::GenRenderbuffers(1, &mut rbo);
            gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
            gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, texture.get_width(), texture.get_height()); // use a single renderbuffer object for both a depth AND stencil buffer.
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::RENDERBUFFER, rbo); // now actually attach it
            // now that we actually created the framebuffer and added all attachments we want to check if it is actually complete now
            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                println!("ERROR::FRAMEBUFFER:: Framebuffer is not complete!");
            }

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        self
    }

    pub fn attach_texture(&mut self, texture: texture::Texture, attach_point: gl::types::GLenum) -> &mut Self {

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
        unsafe {
            
        }
    }

    pub fn get(&self) -> &gl::types::GLuint {
        &self.id
    }
}