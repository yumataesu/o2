use super::traits::Load;
use std::fs::File;
use std::io::Read;
use std::ffi::{CString};

#[derive(Debug, Default)]
pub struct Shader {
    program: gl::types::GLuint,
    vertex_src: CString,
    fragment_src: CString
}

impl Load<&str> for Shader {
    fn load(&mut self, path: &str) {
        let vs_path = &format!("{}.vert", path.clone());
        let fs_path = &format!("{}.frag", path.clone());
        self.vertex_src = self.load_to_cstr(vs_path);//CString::new(vs.as_bytes()).unwrap();
        self.fragment_src = self.load_to_cstr(fs_path);
        self.load();
    }
}

impl Load<(&str, &str)> for Shader {
    fn load(&mut self, path: (&str, &str)) {
        let vs_path = &format!("{}.vert", path.0.clone());
        let fs_path = &format!("{}.frag", path.1.clone());
        self.vertex_src = self.load_to_cstr(vs_path);//CString::new(vs.as_bytes()).unwrap();
        self.fragment_src = self.load_to_cstr(fs_path);
        self.load();
    }
}

impl Load<(&[u8], &[u8])> for Shader {
    fn load(&mut self, str_array: (&[u8], &[u8])) {
        unsafe {
            self.vertex_src = CString::from_vec_unchecked(str_array.0.to_vec());
            self.fragment_src = CString::from_vec_unchecked(str_array.1.to_vec());
            self.load();
        }
    }
}

impl Shader {
    pub fn new() -> Self {
        unsafe {
            let mut program = std::mem::zeroed();
            program = gl::CreateProgram();
            Shader { program: program, vertex_src: CString::new("").expect("CString::new failed"), fragment_src: CString::new("").expect("CString::new failed") }
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

    pub fn uniform_1i(&self, name: &str, v: &i32) {
        unsafe {
            let c_str = CString::new(name).unwrap();
            gl::Uniform1i(gl::GetUniformLocation(self.program, c_str.as_ptr() as *const i8), v.clone());
        }
    }

    pub fn uniform_2i(&self, name: &str, v: &glam::IVec2) {
        unsafe {
            let c_str = CString::new(name).unwrap();
            gl::Uniform2i(gl::GetUniformLocation(self.program, c_str.as_ptr() as *const i8), v.x.clone(), v.y.clone());
        }
    }

    pub fn uniform_3i(&self, name: &str, v: &glam::IVec3) {
        unsafe {
            let c_str = CString::new(name).unwrap();
            gl::Uniform3i(gl::GetUniformLocation(self.program, c_str.as_ptr() as *const i8), v.x.clone(), v.y.clone(), v.z.clone());
        }
    }

    pub fn uniform_4i(&self, name: &str, v: &glam::IVec4) {
        unsafe {
            let c_str = CString::new(name).unwrap();
            gl::Uniform4i(gl::GetUniformLocation(self.program, c_str.as_ptr() as *const i8), v.x.clone(), v.y.clone(), v.z.clone(), v.w.clone());
        }
    }

    pub fn uniform_1f(&self, name: &str, v: &f32) {
        unsafe {
            let c_str = CString::new(name).unwrap();
            gl::Uniform1f(gl::GetUniformLocation(self.program, c_str.as_ptr() as *const i8), v.clone());
        }
    }

    pub fn uniform_2f(&self, name: &str, v: &glam::Vec2) {
        unsafe {
            let c_str = CString::new(name).unwrap();
            gl::Uniform2f(gl::GetUniformLocation(self.program, c_str.as_ptr() as *const i8), v.x.clone(), v.y.clone());
        }
    }

    pub fn uniform_3f(&self, name: &str, v: &glam::Vec3) {
        unsafe {
            let c_str = CString::new(name).unwrap();
            gl::Uniform3f(gl::GetUniformLocation(self.program, c_str.as_ptr() as *const i8), v.x.clone(), v.y.clone(), v.z.clone());
        }
    }

    pub fn uniform_4f(&self, name: &str, v: &glam::Vec4) {
        unsafe {
            let c_str = CString::new(name).unwrap();
            gl::Uniform4f(gl::GetUniformLocation(self.program, c_str.as_ptr() as *const i8), v.x.clone(), v.y.clone(), v.z.clone(), v.w.clone());
        }
    }

    pub fn uniform_mat3(&self, name: &str, mat: &glam::Mat3) {
        unsafe {
            let c_str = CString::new(name).unwrap();
            gl::UniformMatrix4fv(gl::GetUniformLocation(self.program, c_str.as_ptr() as *const i8), 1, gl::FALSE, mat.as_ref().as_ptr());
        }
    }

    pub fn uniform_mat4(&self, name: &str, mat: &glam::Mat4) {
        unsafe {
            let c_str = CString::new(name).unwrap();
            gl::UniformMatrix4fv(gl::GetUniformLocation(self.program, c_str.as_ptr() as *const i8), 1, gl::FALSE, mat.as_ref().as_ptr());
        }
    }



    fn load(&mut self) {
        unsafe {
            let vs = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vs, 1, &self.vertex_src.as_ptr(), std::ptr::null());
            gl::CompileShader(vs);
            self.check_compile_errors(vs, "vertex");

            let fs = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fs, 1, &self.fragment_src.as_ptr(), std::ptr::null());
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

    fn load_to_cstr(&mut self, path: &str) -> CString {
        let mut vs_file = File::open(path)
            .unwrap_or_else(|_| panic!("Failed to open {}", path));

        let mut shader = String::new();
        vs_file.read_to_string(&mut shader)
            .expect("Failed to read shader");

        CString::new(shader.as_bytes()).unwrap()
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


