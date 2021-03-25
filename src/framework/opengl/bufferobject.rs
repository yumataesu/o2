use super::traits::{Allocate, Update};

#[derive(Debug, Copy, Clone)]
pub enum Attribute {
    Index,
    Position,
    Color,
    Texcoord,
    Normal,
}


impl Default for Attribute {
    fn default() -> Self { Attribute::Index }
}


#[derive(Default, Debug)]
pub struct BufferObject {
    id: gl::types::GLuint,
    attribute: Attribute,
    num_verts: i32
}


impl Allocate<(Attribute, &Vec<glam::Vec3>)> for BufferObject {
    fn allocate(&mut self, args: (Attribute, &Vec<glam::Vec3>)) -> &mut Self {
        self.attribute = args.0.clone();
        self.num_verts = args.1.len() as i32;

        unsafe {
            match self.attribute {
                Attribute::Index => {
                },
                _ => {
                    gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
                    gl::BufferData(
                        gl::ARRAY_BUFFER,
                        (args.1.len() * 3 * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                        args.1.as_ptr() as *const _,
                        gl::DYNAMIC_DRAW,
                    );
                }
            }

            self
        }
    }
}


impl Allocate<(Attribute, &Vec<glam::Vec2>)> for BufferObject {
    fn allocate(&mut self, args: (Attribute, &Vec<glam::Vec2>)) -> &mut Self {
        self.attribute = args.0.clone();
        self.num_verts = args.1.len() as i32;

        unsafe {
            match self.attribute {
                Attribute::Index => {
                },
                _ => {
                    gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
                    gl::BufferData(
                        gl::ARRAY_BUFFER,
                        (args.1.len() * 2 * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                        args.1.as_ptr() as *const _,
                        gl::DYNAMIC_DRAW,
                    );
                }
            }
            self
        }
    }
}





impl Allocate<(Attribute, &Vec<u32>)> for BufferObject {
    fn allocate(&mut self, args: (Attribute, &Vec<u32>)) -> &mut Self {
        self.attribute = args.0.clone();
        self.num_verts = args.1.len() as i32;

        unsafe {
            match self.attribute {
                Attribute::Index => {
                    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
                    gl::BufferData(
                        gl::ELEMENT_ARRAY_BUFFER,
                        (args.1.len() * 1 * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                        args.1.as_ptr() as *const _,
                        gl::STATIC_DRAW,
                    );
                },
                _ => ()
            }
            self
        }
    }
}



impl Allocate<(Attribute, &Vec<glam::Vec4>)> for BufferObject {
    fn allocate(&mut self, args: (Attribute, &Vec<glam::Vec4>)) -> &mut Self {
        self.attribute = args.0.clone();
        self.num_verts = args.1.len() as i32;

        unsafe {
            match self.attribute {
                Attribute::Index => {
                },
                _ => {
                    gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
                    gl::BufferData(
                        gl::ARRAY_BUFFER,
                        (args.1.len() * 4 * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                        args.1.as_ptr() as *const _,
                        gl::DYNAMIC_DRAW,
                    );
                }
            }
            self
        }
    }
}


impl Update<(&Vec<glam::Vec3>)> for BufferObject {
    fn update(&mut self, v: &Vec<glam::Vec3>) {
        unsafe {
            gl::NamedBufferSubData(self.id, 0, (v.len() * 3 * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, v.as_ptr() as *const _);
        }
    }
}


impl Update<(&Vec<glam::Vec4>)> for BufferObject {
    fn update(&mut self, v: &Vec<glam::Vec4>) {
        unsafe {
            gl::NamedBufferSubData(self.id, 0, (v.len() * 4 * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, v.as_ptr() as *const _);
        }
    }
}



impl BufferObject {
    pub fn new() -> BufferObject {
        unsafe {
            let mut id = std::mem::zeroed();
            gl::GenBuffers(1, &mut id);
            BufferObject { id: id, attribute: Attribute::Position, num_verts: 0 }
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


    pub fn get_attribute(&self) -> &Attribute {
        &self.attribute
    }

    
    pub fn get_num_verts(&self) -> i32 {
        self.num_verts
    }
}