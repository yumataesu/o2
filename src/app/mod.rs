use crate::framework::{self, opengl};
use imgui_glfw_rs::glfw::{Key, Modifiers, MouseButton};
use imgui_glfw_rs::imgui;
use std::mem;

#[derive(Debug, Default)]
pub struct App {
    number: i32,
    val: f32,
    shader: framework::opengl::shader::Shader,
    id: gl::types::GLuint,
    vao: framework::opengl::vao::Vao,
    position_vbo: framework::opengl::vbo::Vbo,
    color_vbo: framework::opengl::vbo::Vbo
}

impl framework::BaseApp for App {

    fn setup(&mut self) {
        println!("setup");

        self.shader = framework::opengl::shader::Shader::new();
        self.shader.load("data/shader/shader.vert", "data/shader/shader.frag");

        let verts: Vec<f32> = vec![
            -0.5, -0.5, 0.0,  1.0,  0.0,  0.0,
             0.0,  0.5, 0.0,  0.0,  1.0,  0.0,
             0.5, -0.5, 0.0,  0.0,  0.0,  1.0
        ];


        let vetices: Vec<f32> = vec![
            -0.5, -0.5, 0.0,
             0.0,  0.5, 0.0,
             0.5, -0.5, 0.0
        ];

        let colors: Vec<f32> = vec![
            0.5, 0.5, 0.0,
             0.0,  0.5, 0.0,
             0.5, 0.0, 0.0
        ];

        self.position_vbo = framework::opengl::vbo::Vbo::new();
        self.position_vbo.allocate(&vetices);

        // self.color_vbo = framework::opengl::vbo::Vbo::new();
        // self.color_vbo.allocate(&colors);

        self.vao = framework::opengl::vao::Vao::new();
        self.vao.set_position_vbo(&self.position_vbo);
        // self.vao.set_color_vbo(&self.color_vbo);

        // unsafe {
        //     //if gl::BindVertexArray.is_loaded() {
        //     self.id = std::mem::zeroed();
        //     gl::GenVertexArrays(1, &mut self.id);
        //     gl::BindVertexArray(self.id);
        //     //}//
    
        //     let pos_attrib = gl::GetAttribLocation(self.shader.get_program(), b"position\0".as_ptr() as *const _);
        //     let color_attrib = gl::GetAttribLocation(self.shader.get_program(), b"color\0".as_ptr() as *const _);
        //     gl::VertexAttribPointer(
        //         pos_attrib as gl::types::GLuint,
        //         3,
        //         gl::FLOAT,
        //         0,
        //         6 * std::mem::size_of::<f32>() as gl::types::GLsizei,
        //         std::ptr::null(),
        //     );
        //     gl::VertexAttribPointer(
        //         color_attrib as gl::types::GLuint,
        //         3,
        //         gl::FLOAT,
        //         0,
        //         6 * std::mem::size_of::<f32>() as gl::types::GLsizei,
        //         (3 * std::mem::size_of::<f32>()) as *const () as *const _,
        //     );
        //     gl::EnableVertexAttribArray(pos_attrib as gl::types::GLuint);
        //     gl::EnableVertexAttribArray(color_attrib as gl::types::GLuint);
        // }
    }


    fn update(&mut self) {

    }


    fn draw(&mut self) {
        framework::opengl::utils::clear_color(0.05, 0.05, 0.05, 1.0);
        framework::opengl::utils::clear();
        self.shader.begin();
        //self.vao.draw();
        // unsafe {
        //     gl::BindVertexArray(self.id);
        //     gl::DrawArrays(gl::TRIANGLES, 0, 3);
        // }
        self.shader.end();
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