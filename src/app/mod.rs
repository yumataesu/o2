use crate::framework;
use glfw::{Key, Modifiers};


#[derive(Debug, Default)]
pub struct App {
    number: i32
}

impl framework::BaseApp for App {

    fn setup(&self) {
        println!("setup");

        let vs_src = std::fs::read_to_string("data/shader/shader.vert").expect("Something went wrong reading the file");
        let fs_src = std::fs::read_to_string("data/shader/shader.frag").expect("Something went wrong reading the file");
        let program = unsafe { gl::CreateProgram() };
        unsafe {
            let vs = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vs, 1, [vs_src.as_bytes().as_ptr() as *const _].as_ptr(), std::ptr::null());
            gl::CompileShader(vs);

            let fs = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fs, 1, [fs_src.as_bytes().as_ptr() as *const _].as_ptr(), std::ptr::null());
            gl::CompileShader(fs);

            gl::AttachShader(program, vs);
            gl::AttachShader(program, fs);
            gl::LinkProgram(program);
        }
    }


    fn update(&self) {

    }


    fn draw(&self) {
        unsafe {
            gl::ClearColor(0.0, 1.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }


    fn key_pressed(&self, key: Key, modifiers: Modifiers) {
        println!("key_pressed {:?}", key);
    }


    fn key_released(&self, key: Key, modifiers: Modifiers) {
        println!("key_released {:?}", key);
    }
}