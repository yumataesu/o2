// // mod traits;
// mod self::traits;
use super::traits::Load;

#[derive(Debug, Default)]
pub struct Shader {
    program: gl::types::GLuint,
    vertex_src: String,
    fragment_src: String
}

impl Load<&str> for Shader {
    fn load(&mut self, path: &str) {
        self.vertex_src = std::fs::read_to_string(format!("{}.vert", path).to_string())
            .expect("Something went wrong reading the file");
        self.fragment_src = std::fs::read_to_string(format!("{}.frag", path).to_string())
            .expect("Something went wrong reading the file");

        self.load();
    }
}

impl Load<(&str, &str)> for Shader {
    fn load(&mut self, path: (&str, &str)) {
        self.vertex_src = std::fs::read_to_string(path.0)
            .expect("Something went wrong reading the file");
        self.fragment_src = std::fs::read_to_string(path.1)
            .expect("Something went wrong reading the file");
            
        self.load();
    }
}

impl Shader {
    pub fn new() -> Self {
        unsafe {
            Shader { program: gl::CreateProgram(), vertex_src: String::new(), fragment_src: String::new() }
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
        unsafe {
            self.program
        }
    }


    fn load(&mut self) {
        unsafe {
            let vs = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vs, 1, [self.vertex_src.as_bytes().as_ptr() as *const _].as_ptr(), std::ptr::null());
            gl::CompileShader(vs);

            let fs = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fs, 1, [self.fragment_src.as_bytes().as_ptr() as *const _].as_ptr(), std::ptr::null());
            gl::CompileShader(fs);

            gl::AttachShader(self.program, vs);
            gl::AttachShader(self.program, fs);
            gl::LinkProgram(self.program);
        }
    }
}


