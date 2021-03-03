use crate::framework;

#[derive(Debug, Default)]
pub struct Shader {
    program: gl::types::GLuint
}

impl Shader {
    pub fn new() -> Self {
        unsafe {
            Shader { program: gl::CreateProgram() }
        }
    }

    pub fn load(&mut self, vertex_shader_path: &str, fragment_shader_path: &str) {
        let vs_src = std::fs::read_to_string(vertex_shader_path).expect("Something went wrong reading the file");
        let fs_src = std::fs::read_to_string(fragment_shader_path).expect("Something went wrong reading the file");
        unsafe {
            let vs = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vs, 1, [vs_src.as_bytes().as_ptr() as *const _].as_ptr(), std::ptr::null());
            gl::CompileShader(vs);

            let fs = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fs, 1, [fs_src.as_bytes().as_ptr() as *const _].as_ptr(), std::ptr::null());
            gl::CompileShader(fs);

            gl::AttachShader(self.program, vs);
            gl::AttachShader(self.program, fs);
            gl::LinkProgram(self.program);
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

    pub fn get_program(&self) -> gl::types::GLuint {
        unsafe {
            self.program
        }
    }
}


