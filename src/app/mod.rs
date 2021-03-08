use crate::framework::{self};
use imgui_glfw_rs::glfw::{Key, Modifiers, MouseButton};
use imgui_glfw_rs::imgui;
use framework::Load;
use framework::Allocate;

use rand::distributions::*;
use glam::Vec3;

#[derive(Debug, Default)]
pub struct App {
    val: f32,
    shader: framework::Shader,
    vao: framework::Vao,
    position_vbo: framework::Vbo,
    color_vbo: framework::Vbo,
    positions: Vec<glam::Vec3>,
    colors: Vec<glam::Vec3>,
    num: usize
}

impl framework::BaseApp for App {

    fn setup(&mut self) {
        println!("setup");
        self.shader = framework::Shader::new();
        self.shader.load("data/shader/shader");

        self.num = 10;
        let prange = rand::distributions::Uniform::new(-1.0f32, 1.0);
        let crange = rand::distributions::Uniform::new(0.0f32, 1.0);
        let mut rng = rand::thread_rng();
        
        self.positions = Vec::with_capacity(self.num);
        self.colors = Vec::with_capacity(self.num);
        for i in 0..self.num {
            self.positions.push(glam::Vec3::new(prange.sample(&mut rng), prange.sample(&mut rng), 0.0));
            self.colors.push(glam::Vec3::new(crange.sample(&mut rng), crange.sample(&mut rng), crange.sample(&mut rng)));
        }

        self.position_vbo = framework::Vbo::new();
        self.position_vbo.allocate(&self.positions);

        self.color_vbo = framework::Vbo::new();
        self.color_vbo.allocate(&self.colors);

        self.vao = framework::Vao::new();
        self.vao.set_vbo(framework::VertexAttribute::Position, &self.position_vbo);
        self.vao.set_vbo(framework::VertexAttribute::Color, &self.color_vbo);
    }


    fn update(&mut self) {
        self.positions.clear();

        let prange = rand::distributions::Uniform::new(-1.0f32, 1.0);
        let mut rng = rand::thread_rng();
        for i in 0..self.num {
            self.positions.push(glam::Vec3::new(prange.sample(&mut rng), prange.sample(&mut rng), 0.0));
        }

        self.position_vbo.update(&self.positions);
    }


    fn draw(&mut self) {
        framework::gl_utils::clear_color(0.1, 0.1, 0.1, 1.0);
        framework::gl_utils::clear();
        self.shader.begin();
        self.vao.draw();
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