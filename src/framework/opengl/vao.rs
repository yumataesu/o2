use super::vbo;
#[derive(Debug, Copy, Clone)]
pub enum VertexAttribute {
    Position,
    Color,
    Texcoord,
    Normal
}
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

    pub fn set_vbo(&self, attribute_type: VertexAttribute, vbo: &vbo::Vbo) {
        let mut num: i32;
        let location = attribute_type.clone() as gl::types::GLuint;
        unsafe {
            gl::BindVertexArray(self.id);
            gl::BindBuffer(gl::ARRAY_BUFFER, *vbo.get());
            gl::EnableVertexAttribArray(location);
            match attribute_type {
                VertexAttribute::Position => {
                    num = 3;
                },
                VertexAttribute::Color => {
                    num = 3;
                },
                VertexAttribute::Texcoord => {
                    num = 2;
                },
                VertexAttribute::Normal => {
                    num = 3;
                },
            }
            gl::VertexAttribPointer(
                location,
                num,
                gl::FLOAT,
                0,
                num * std::mem::size_of::<f32>() as gl::types::GLsizei,
                std::ptr::null());
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