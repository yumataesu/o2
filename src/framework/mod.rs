use gl;

use imgui_glfw_rs::glfw::{self, Context};
use imgui_glfw_rs::imgui;
use imgui_glfw_rs::ImguiGLFW;

pub mod util;
pub mod settings;
pub mod opengl;

use crate::app::App;

pub trait BaseApp {
    fn setup(&mut self);
    fn update(&mut self);
    fn draw(&mut self);
    fn draw_gui(&mut self, ui: &imgui_glfw_rs::imgui::Ui);
    fn key_pressed(&mut self, key: glfw::Key, modifiers: glfw::Modifiers);
    fn key_released(&mut self, key: glfw::Key, modifiers: glfw::Modifiers);
    fn mouse_pressed(&mut self, button: glfw::MouseButton);
    fn mouse_released(&mut self, button: glfw::MouseButton);
    fn cursor_moved(&mut self, x: f64, y: f64);
    fn file_dropped(&mut self, paths: Vec<std::path::PathBuf>);
}


pub struct Runner {
    app : App,
    window: glfw::Window,
    events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
    glfw: glfw::Glfw,
    window_settings: settings::WindowSettings,
    frame_rate : f64,
    last_time: std::time::Instant,
    imgui: imgui::Context,
    imgui_glfw: ImguiGLFW
}


impl Runner {
    pub fn new(mut app: App, ws: settings::WindowSettings) -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(ws.gl_version.0.clone(), ws.gl_version.1.clone()));

        let (mut window, events) = glfw
        .create_window(
            ws.window_size.0.clone(),
            ws.window_size.1.clone(),
            &ws.title,
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create window");

        window.make_current();
        window.set_all_polling(true);

        unsafe {
            gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        }

        let mut imgui = imgui::Context::create();
        let imgui_glfw = ImguiGLFW::new(&mut imgui, &mut window);

        
        Runner { 
            app: app, 
            window: window, 
            events: events, 
            glfw: glfw,
            window_settings: ws,
            frame_rate: 60.0 as f64, 
            last_time: std::time::Instant::now(),
            imgui: imgui,
            imgui_glfw: imgui_glfw
        }
    }


    pub fn run(&mut self) {
        self.app.setup();

        while !self.window.should_close() {
            self.last_time = std::time::Instant::now();
            let millisec_at_fps = std::time::Duration::from_millis((1.0 / self.frame_rate.clone() * 1000.0) as u64);

            self.app.update();
            self.app.draw();


            let ui = self.imgui_glfw.frame(&mut self.window, &mut self.imgui);
            self.app.draw_gui(&ui);
            self.imgui_glfw.draw(ui, &mut self.window);


            self.window.swap_buffers();

            //event
            self.glfw.poll_events();
            for (_, event) in glfw::flush_messages(&self.events) {
                self.imgui_glfw.handle_event(&mut self.imgui, &event);
                match event {
                    glfw::WindowEvent::Key(key, _, action, modifiers) => {
                        match action {
                            glfw::Action::Press => {
                                match key {
                                    glfw::Key::Escape => { self.window.set_should_close(true); },
                                    _ => { self.app.key_pressed(key, modifiers); }
                                }
                            },
                            glfw::Action::Release => {
                                match key {
                                    _ => { self.app.key_released(key, modifiers); }
                                }
                            },
                            glfw::Action::Repeat => {}
                            _ => {}
                        }
                    },
                    glfw::WindowEvent::MouseButton(button, action, modifiers) => {
                        match action {
                            glfw::Action::Press => { self.app.mouse_pressed(button); },
                            glfw::Action::Release => { self.app.mouse_released(button); },
                            glfw::Action::Repeat => {}
                        }
                    },
                    glfw::WindowEvent::CursorPos(x, y) => { self.app.cursor_moved(x, y); },
                    glfw::WindowEvent::FileDrop(paths) => { self.app.file_dropped(paths); }
                    _ => {}
                }
            }

            let duration = std::time::Instant::now().duration_since(self.last_time);
            if duration < millisec_at_fps {
                std::thread::sleep(millisec_at_fps - duration);
            }
        }
    }
}