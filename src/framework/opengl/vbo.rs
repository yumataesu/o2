use super::traits::Allocate;
use glam::Vec3;

#[derive(Debug, Default)]
pub struct Vbo {
    id: gl::types::GLuint,
    vertices: Vec<f32>,
    num_verts: i32
}


impl Allocate<&Vec<glam::Vec3>> for Vbo {
    fn allocate(&mut self, v: &Vec<glam::Vec3>) {
        self.num_verts = v.len() as i32;
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (v.len() * 3 * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                v.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
        }
    }
}

impl Allocate<&Vec<glam::Vec4>> for Vbo {
    fn allocate(&mut self, v: &Vec<glam::Vec4>) {
        self.num_verts = v.len() as i32;
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (v.len() * 4 * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                v.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
        }
    }
}



impl Vbo {
    pub fn new() -> Vbo {
        unsafe {
            let mut id = std::mem::zeroed();
            gl::GenBuffers(1, &mut id);
            Vbo { id: id, vertices: Vec::new(), num_verts: 0 }
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

    pub fn get(&self) -> &gl::types::GLuint {
        &self.id
    }

    pub fn get_vertices(&self) -> &Vec<f32> {
        &self.vertices
    }

    pub fn get_num_verts(&self) -> i32 {
        self.num_verts
    }
}