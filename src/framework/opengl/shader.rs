use super::traits::Load;
use glam::Mat4;
use std::ffi::{CString, CStr};
const VS_SRC: &'static [u8] = b"
#version 450

layout (location = 0) in vec3 position;
layout (location = 1) in vec4 color;
layout (location = 2) in vec2 texcoord;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;


out vec4 v_color;
out vec2 v_texcoord;

void main() {
    gl_Position = projection * view * model * vec4(position, 1.0);
    //gl_Position = vec4(position, 1.0);

    v_color = color;
    v_texcoord = texcoord;
}
\0";

const FS_SRC: &'static [u8] = b"
#version 450

in vec4 v_color;
in vec2 v_texcoord;

uniform sampler2D u_src;

layout (location = 0) out vec4 FragColor;

void main() {
    vec4 result = texture(u_src, v_texcoord);
    FragColor = result;
}
\0";


#[derive(Debug, Default)]
pub struct Shader {
    program: gl::types::GLuint,
    vertex_src: String,
    fragment_src: String
}

impl Load<&str> for Shader {
    fn load(&mut self, path: &str) {
        // self.vertex_src = std::fs::read_to_string(format!("{}.vert", path).to_string())
        //     .expect("Something went wrong reading the file");
        // self.fragment_src = std::fs::read_to_string(format!("{}.frag", path).to_string())
        //     .expect("Something went wrong reading the file");

        self.load();
    }
}

impl Load<(&str, &str)> for Shader {
    fn load(&mut self, path: (&str, &str)) {
        // self.vertex_src = std::fs::read_to_string(path.0)
        //     .expect("Something went wrong reading the file");
        // self.fragment_src = std::fs::read_to_string(path.1)
        //     .expect("Something went wrong reading the file");
            
        self.load();
    }
}

impl Shader {
    pub fn new() -> Self {
        unsafe {
            let mut program = std::mem::zeroed();
            program = gl::CreateProgram();
            Shader { program: program, vertex_src: String::new(), fragment_src: String::new() }
        }
    }

    
    pub fn begin(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }
    
    pub fn end(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }
    
    pub fn get(&self) -> gl::types::GLuint {
        self.program
    }

    pub fn uniform_texture(&self, name: &str, id: &gl::types::GLuint) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, *id);
        }
    }

    pub fn uniform_mat4(&self, name: &CStr, mat: &glam::Mat4) {
        unsafe {
            // println!(" : {}", gl::GetUniformLocation(self.program, name.as_ptr()));
            gl::UniformMatrix4fv(gl::GetUniformLocation(self.program, name.as_ptr()), 1, gl::FALSE, mat.as_ref().as_ptr());
        }
    }


    fn load(&mut self) {
        //println!("self.program {}", self.program);
        //println!("self.vertex_src {}", self.vertex_src);
        //println!("self.fragment {}", self.fragment_src);
        
        unsafe {
            let vs = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vs, 1, [VS_SRC.as_ptr() as *const _].as_ptr(), std::ptr::null());
            gl::CompileShader(vs);
            self.check_compile_errors(vs, "vertex");

            let fs = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fs, 1, [FS_SRC.as_ptr() as *const _].as_ptr(), std::ptr::null());
            gl::CompileShader(fs);
            self.check_compile_errors(fs, "fragment");

            gl::AttachShader(self.program, vs);
            gl::AttachShader(self.program, fs);
            gl::LinkProgram(self.program);

            //check program
            self.check_compile_errors(self.program, "PROGRAM");

            gl::DeleteShader(vs);
            gl::DeleteShader(fs);
        }
    }


    /// utility function for checking shader compilation/linking errors.
    /// https://github.com/bwasty/learn-opengl-rs/blob/master/src/shader.rs#L102
    /// ------------------------------------------------------------------------
    unsafe fn check_compile_errors(&self, shader: u32, type_: &str) {
        let mut success = gl::FALSE as gl::types::GLint;
        let mut infoLog = Vec::with_capacity(2048);
        infoLog.set_len(2048 - 1); // subtract 1 to skip the trailing null character
        if type_ != "PROGRAM" {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as gl::types::GLint {
                gl::GetShaderInfoLog(shader, 2048, std::ptr::null_mut(), infoLog.as_mut_ptr() as *mut gl::types::GLchar);
                println!("ERROR::SHADER_COMPILATION_ERROR of type: {}\n{:?}\n \
                          -- --------------------------------------------------- -- ",
                         type_,
                         std::str::from_utf8(&infoLog).expect("Found invalid UTF-8"));
            }

        } else {
            gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as gl::types::GLint {
                gl::GetProgramInfoLog(shader, 2048, std::ptr::null_mut(), infoLog.as_mut_ptr() as *mut gl::types::GLchar);
                println!("ERROR::PROGRAM_LINKING_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                         type_,
                         std::str::from_utf8(&infoLog).expect("Found invalid UTF-8"));
            }
        }
    }
}


