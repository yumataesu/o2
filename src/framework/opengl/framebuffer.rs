use super::traits::{Allocate, Update};
use super::texture;

#[derive(Debug, Default)]
pub struct FrameBuffer {
    id: gl::types::GLuint,
    width: i32,
    height: i32,
    texture: texture::Texture,
    color_buffers: Vec<gl::types::GLuint>,
    textures: Vec<texture::Texture>,
    is_allocated_rbo: bool
}


impl Allocate<(i32, i32, i32, gl::types::GLenum)> for FrameBuffer {
    fn allocate(&mut self, args: (i32, i32, i32, gl::types::GLenum)) -> &mut Self {
        unsafe {
            let w = args.0;
            let h = args.1;
            let internal_format = args.2;
            let attach_point = args.3;

            let mut tex = texture::Texture::new();
            tex.allocate((w, h, internal_format));
            self.textures.push(tex);

            self.allocate(&mut self.textures[0], gl::COLOR_ATTACHMENT0)
        }
    }
}


impl FrameBuffer {
    pub fn new() -> FrameBuffer {
        unsafe {
            let mut id = std::mem::zeroed();
            gl::GenFramebuffers(1, &mut id);
            FrameBuffer { id: id, width: 512, height:512, texture: texture::Texture::new(), color_buffers: Vec::new(), textures: Vec::new(), is_allocated_rbo: false}
        }
    }

    fn allocate(&mut self, texture: &mut texture::Texture, attach_point: gl::types::GLenum) -> &mut Self {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, attach_point, gl::TEXTURE_2D, *texture.get(), 0);

            if (!self.is_allocated_rbo) {
                let mut rbo = 0;
                gl::GenRenderbuffers(1, &mut rbo);
                gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);
                gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, texture.get_width(), texture.get_height()); // use a single renderbuffer object for both a depth AND stencil buffer.
                gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::RENDERBUFFER, rbo); // now actually attach it
                // now that we actually created the framebuffer and added all attachments we want to check if it is actually complete now
                self.is_allocated_rbo = true;
            }

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
        &self.textures[0].get()
    }
}