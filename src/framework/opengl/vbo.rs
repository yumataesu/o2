#[derive(Debug, Default)]
pub struct Vbo {
    id: gl::types::GLuint,
    vertices: Vec<f32>
}

impl Vbo {
    pub fn new() -> Vbo {
        unsafe {
            let mut id = std::mem::zeroed();
            gl::GenBuffers(1, &mut id);
            Vbo { id: id, vertices: Vec::new() }
        }
    }

    pub fn allocate(&mut self, v: &Vec<f32>) {
        unsafe {
            // self.vertices = v;
            println!("allocate {}", self.id);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (v.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                v.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
        }
    }



    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn get_vbo(&self) -> &gl::types::GLuint {
        unsafe {
            &self.id
        }
    }

    pub fn get_vertices(&self) -> &Vec<f32> {
        &self.vertices
    }
}