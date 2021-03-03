#[derive(Debug, Default)]
pub struct Vbo {
    id: gl::types::GLuint
}


impl Vbo {
    pub fn new() -> Vbo {
        unsafe {
            let mut id = std::mem::zeroed();
            gl::GenBuffers(1, &mut id);
            Vbo { id: id }
        }
    }

    pub fn allocate(&mut self, v: Vec<f32>) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (v.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                v.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
        }
    }

    pub fn get_vbo(&self) -> &gl::types::GLuint {
        unsafe {
            &self.id
        }
    }
}