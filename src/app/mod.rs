use crate::framework::{self};
use imgui_glfw_rs::glfw::{Key, Modifiers, MouseButton};
use imgui_glfw_rs::imgui;
use framework::{Load, Allocate, Update};

use rand::distributions::*;
use glam::Vec3;

#[derive(Debug, Default)]
pub struct App {
    val: f32,
    shader: framework::Shader,
    vao: framework::Vao,
    position_vbo: framework::BufferObject,
    color_vbo: framework::BufferObject,
    positions: Vec<glam::Vec3>,
    vel: Vec<glam::Vec3>,
    acc: Vec<glam::Vec3>,
    colors: Vec<glam::Vec3>,
    num: usize,
    center: glam::Vec3
}

impl framework::BaseApp for App {

    fn setup(&mut self) {
        println!("setup");
        self.shader = framework::Shader::new();
        self.shader.load("data/shader/shader");

        self.num = 100000;
        let prange = rand::distributions::Uniform::new(-1.0f32, 1.0);
        let crange = rand::distributions::Uniform::new(0.0f32, 1.0);
        let mut rng = rand::thread_rng();
        
        self.positions = Vec::with_capacity(self.num);
        self.colors = Vec::with_capacity(self.num);
        for i in 0..self.num {
            self.positions.push(glam::Vec3::new(prange.sample(&mut rng), prange.sample(&mut rng), 0.0));
            self.vel.push(glam::Vec3::new(0.0, 0.0, 0.0));
            self.acc.push(glam::Vec3::new(0.0, 0.0, 0.0));
            self.colors.push(glam::Vec3::new(crange.sample(&mut rng), crange.sample(&mut rng), 1.0));
        }

        self.position_vbo = framework::BufferObject::new();
        self.position_vbo.allocate((framework::VertexAttribute::Position, &self.positions));

        self.color_vbo = framework::BufferObject::new();
        self.color_vbo.allocate((framework::VertexAttribute::Color, &self.positions));

        self.vao = framework::Vao::new();
        self.vao.set_vbo(&self.position_vbo);
        self.vao.set_vbo(&self.color_vbo);
    }


    fn update(&mut self) {
        for i in 0..self.num {
            self.acc[i] = glam::Vec3::new(0.0,0.0,0.0);

            self.acc[i] = self.center - self.positions[i];
            self.acc[i] = self.acc[i].normalize()* 0.1;
            self.vel[i] += self.acc[i] * 0.001;
            self.positions[i] += self.vel[i];
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

    fn cursor_moved(&mut self, x: f32, y: f32) {
        // println!("cursor_moved {}, {}", x, y);
        self.center = glam::Vec3::new(x / 1920.0f32 * 2.0 - 1.0, 1.0 - (y / 1080.0f32 * 2.0 - 1.0), 0.0);
    }
}