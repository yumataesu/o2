use super::bufferobject;


#[derive(Debug, Default)]
pub struct Vao {
    id: gl::types::GLuint,
    num_vertex: i32
}


impl Vao {
    pub fn new() -> Vao {
        unsafe {
            let mut id = std::mem::zeroed();
            gl::GenVertexArrays(1, &mut id);
            Vao { id: id, num_vertex: 0 }
        }
    }

    
    pub fn set_vbo(&mut self, vbo: &bufferobject::BufferObject) {
        let mut num: i32;
        let mut location: u32;

        unsafe {
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
                    return;
                }
            }
            //let location = vbo.get_attribute().clone() as gl::types::GLuint;
            self.num_vertex = vbo.get_num_verts();
            gl::BindVertexArray(self.id);
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


    pub fn set_ebo(&mut self, ebo: &bufferobject::BufferObject) {
        unsafe {
            gl::BindVertexArray(self.id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, *ebo.get());
        }
    }


    pub fn get(&self) -> &gl::types::GLuint {
        &self.id
    }


    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
            gl::DrawArrays(gl::POINTS, 0, self.num_vertex);
            gl::BindVertexArray(0);
        }
    }
}