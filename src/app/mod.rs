use crate::framework::{self};
use imgui_glfw_rs::glfw::{Key, Modifiers, MouseButton};
use imgui_glfw_rs::imgui;

#[derive(Debug, Default)]
pub struct App {
    number: i32,
    val: f32,
    shader: framework::Shader,
    vao: framework::Vao,
    position_vbo: framework::Vbo,
    color_vbo: framework::Vbo
}

impl framework::BaseApp for App {

    fn setup(&mut self) {
        println!("setup");

        self.shader = framework::Shader::new();
        self.shader.load("data/shader/shader.vert", "data/shader/shader.frag");

        let vetices: Vec<f32> = vec![
            -0.5, -0.5,  0.0,
             0.0,  0.5,  0.0,
             0.5, -0.5,  0.0
        ];

        let colors: Vec<f32> = vec![
             1.0, 0.0, 0.0,
             0.0, 1.0, 0.0,
             0.0, 0.0, 1.0
        ];

        self.position_vbo = framework::Vbo::new();
        self.position_vbo.allocate(vetices);

        self.color_vbo = framework::Vbo::new();
        self.color_vbo.allocate(colors);

        self.vao = framework::Vao::new();
        self.vao.set_position_vbo(&self.position_vbo);
        self.vao.set_color_vbo(&self.color_vbo);
    }


    fn update(&mut self) {

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