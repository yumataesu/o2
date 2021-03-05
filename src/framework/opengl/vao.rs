use super::vbo;

#[derive(Debug, Default)]
pub struct Vao {
    id: gl::types::GLuint
}

impl Vao {
    pub fn new() -> Vao {
        unsafe {
            let mut id = std::mem::zeroed();
            gl::GenVertexArrays(1, &mut id);
            Vao { id: id }
        }
    }

    pub fn set_position_vbo(&self, vbo: &vbo::Vbo) {
        unsafe {
            gl::BindVertexArray(self.id);
            gl::BindBuffer(gl::ARRAY_BUFFER, *vbo.get());
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                0,
                3 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                std::ptr::null());
        }
    }

    pub fn set_color_vbo(&self, vbo: &vbo::Vbo) {
        unsafe {
            gl::BindVertexArray(self.id);
            gl::BindBuffer(gl::ARRAY_BUFFER, *vbo.get());
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                0,
                3 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                std::ptr::null(),
            );
        }
    }

    pub fn get(&self) -> &gl::types::GLuint {
        &self.id
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::BindVertexArray(0);
        }
    }
}