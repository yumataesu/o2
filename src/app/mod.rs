use crate::framework::{self, opengl};
use imgui_glfw_rs::glfw::{Key, Modifiers, MouseButton};
use imgui_glfw_rs::imgui;
use std::mem;

#[derive(Debug, Default)]
pub struct App {
    number: i32,
    val: f32,
    shader: framework::opengl::shader::Shader,
    vao: gl::types::GLuint,
    vbo: framework::opengl::vbo::Vbo
}

// #[derive(Debug, Default)]
// pub struct Test {
//     num: f32
// }

// fn do_fn_once<F: FnOnce()>(f: F) {
//     f();
// }

// fn do_fn<F: Fn()>(f: F) {
//     f();
//     f();
// }

// impl App {
//     fn test(&self) {
//         println!("test {}", self.number);
//     }
// }

// fn do_mut<F: FnMut()>(f: F) {
//     f();
// }

impl framework::BaseApp for App {

    fn setup(&mut self) {
        println!("setup");

        let arg = String::from("Args");
        let num = String::from("Num");

        println!("num: {:p}", &num);
        println!("arg: {:p}", &arg);

        let add = |arg| {
            println!("arg in clojure: {:p}", &arg);
            println!("num in clojure: {:p}", &num);
        };

        let f = || {
            println!("num in fnonce clojure: {:p}", &num);
        };

        add(&arg);
        println!("arg: {:p}", &arg);
        println!("num: {:p}", &num);
        // do_fn_once(f);
        // self.test();

        f();

        self.shader = framework::opengl::shader::Shader::new();
        self.shader.load("data/shader/shader.vert", "data/shader/shader.frag");

        let verts: Vec<f32> = vec![
            -0.5, -0.5,  1.0,  0.0,  0.0,
             0.0,  0.5,  0.0,  1.0,  0.0,
             0.5, -0.5,  0.0,  0.0,  1.0
        ];
        // self.vbo = framework::opengl::vbo::Vbo::new();
        // self.vbo.allocate(verts);

        unsafe {
            let mut vb = std::mem::zeroed();
            gl::GenBuffers(1, &mut vb);
            gl::BindBuffer(gl::ARRAY_BUFFER, vb);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (verts.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                verts.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
    
            //if gl::BindVertexArray.is_loaded() {
            self.vao = std::mem::zeroed();
            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);
            //}//
    
            let pos_attrib = gl::GetAttribLocation(self.shader.get_program(), b"position\0".as_ptr() as *const _);
            let color_attrib = gl::GetAttribLocation(self.shader.get_program(), b"color\0".as_ptr() as *const _);
            gl::VertexAttribPointer(
                pos_attrib as gl::types::GLuint,
                2,
                gl::FLOAT,
                0,
                5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                std::ptr::null(),
            );
            gl::VertexAttribPointer(
                color_attrib as gl::types::GLuint,
                3,
                gl::FLOAT,
                0,
                5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                (2 * std::mem::size_of::<f32>()) as *const () as *const _,
            );
            gl::EnableVertexAttribArray(pos_attrib as gl::types::GLuint);
            gl::EnableVertexAttribArray(color_attrib as gl::types::GLuint);
        }


    }


    fn update(&mut self) {

    }


    fn draw(&mut self) {
        framework::opengl::utils::clear_color(0.05, 0.05, 0.05, 1.0);
        framework::opengl::utils::clear();
        unsafe {
            self.shader.begin();
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            self.shader.end();

        }
    }


    fn draw_gui(&mut self, ui: &imgui::Ui) {
        ui.show_demo_window(&mut true);
        let win = ui.window(imgui::im_str!("title"));
        ui.slider_float(imgui::im_str!("u8 value"), &mut self.val, -1.0, 1.0).build();
    }


    fn key_pressed(&mut self, key: Key, modifiers: Modifiers) {
        // println!("key_pressed {:?}", key);
    }

    fn key_released(&mut self, key: Key, modifiers: Modifiers) {
        // println!("key_released {:?}", key);
    }

    fn mouse_pressed(&mut self, button: MouseButton) {
        // println!("mouse_pressed {:?}", button);
    }

    fn mouse_released(&mut self, button: MouseButton) {
        // println!("mouse_released {:?}", button);
    }

    fn file_dropped(&mut self, paths: Vec<std::path::PathBuf>) {
        // println!("file_dropped {:?}", paths);
    }

    fn cursor_moved(&mut self, x: f64, y: f64) {
        // println!("cursor_moved {}, {}", x, y);
    }
}