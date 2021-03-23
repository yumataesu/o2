use super::traits::{Allocate, Update};
use super::bufferobject;

#[derive(Debug, Default)]
pub struct Vao {
    id: gl::types::GLuint,
    num_indices: i32,
    num_vertex: i32
}


impl Vao {
    pub fn new() -> Vao {
        unsafe {
            let mut id = std::mem::zeroed();
            gl::GenVertexArrays(1, &mut id);
            Vao { id: id, num_vertex: 0, num_indices: 0 }
        }
    }

    
    pub fn set_vbo(&mut self, vbo: &bufferobject::BufferObject) {
        let mut num: i32;
        let mut location: u32;
        self.num_vertex = vbo.get_num_verts();
        unsafe {
            gl::BindVertexArray(self.id);
            match vbo.get_attribute() {
                bufferobject::Attribute::Position => {
                    num = 3;
                    location = 0;
                },
                bufferobject::Attribute::Color => {
                    num = 4;
                    location = 1;
                },
                bufferobject::Attribute::Texcoord => {
                    num = 2;
                    location = 2;
                },
                bufferobject::Attribute::Normal => {
                    num = 3;
                    location = 3;
                },
                bufferobject::Attribute::Index => {
                    self.num_indices = vbo.get_num_verts();
                    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, *vbo.get());
                    return;
                }
            }

            gl::BindBuffer(gl::ARRAY_BUFFER, *vbo.get());
            gl::EnableVertexAttribArray(location);
            gl::VertexAttribPointer(
                location,
                num,
                gl::FLOAT,
                0,
                num * std::mem::size_of::<f32>() as gl::types::GLsizei,
                std::ptr::null());
        }
    }

    
    pub fn create_quad(&self) {
        let mut position_vbo = bufferobject::BufferObject::new();
        let mut texcoord_vbo = bufferobject::BufferObject::new();
        let mut ebo = bufferobject::BufferObject::new();

        let w = 1.0; let h = 1.0;
        let mut positions = Vec::new();
        let mut texcoords = Vec::new();
        let mut indices = Vec::new();

        positions.push(glam::Vec3::new(-w, -h, 0.0));
        positions.push(glam::Vec3::new(w, -h, 0.0));
        positions.push(glam::Vec3::new(w, h, 0.0));
        positions.push(glam::Vec3::new(-w, h, 0.0));

        texcoords.push(glam::Vec2::new(0.0, 1.0));
        texcoords.push(glam::Vec2::new(1.0, 1.0));
        texcoords.push(glam::Vec2::new(1.0, 0.0));
        texcoords.push(glam::Vec2::new(0.0, 0.0));

        indices.push(0);
        indices.push(1);
        indices.push(2);
        indices.push(0);
        indices.push(3);
        indices.push(2);

        position_vbo.allocate((bufferobject::Attribute::Position, &positions));
        texcoord_vbo.allocate((bufferobject::Attribute::Texcoord, &texcoords));
        ebo.allocate((bufferobject::Attribute::Index, &indices));
    }



    pub fn get(&self) -> &gl::types::GLuint {
        &self.id
    }


    pub fn draw(&self, draw_type: gl::types::GLenum) {
        unsafe {
            gl::BindVertexArray(self.id);
            gl::DrawArrays(draw_type, 0, self.num_vertex);
            gl::BindVertexArray(0);
        }
    }

    pub fn draw_elements(&self, draw_type: gl::types::GLenum) {
        unsafe {
            gl::BindVertexArray(self.id);
            gl::DrawElements(draw_type, self.num_indices, gl::UNSIGNED_INT, std::ptr::null());
            gl::BindVertexArray(0);
        }
    }
}