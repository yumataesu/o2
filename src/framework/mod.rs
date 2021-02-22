extern crate gl;
extern crate glfw;
use glfw::{Action, Context, Key, Modifiers};

use crate::app::App;

pub trait BaseApp {
    fn setup(&self);
    fn update(&self);
    fn draw(&self);
    fn key_pressed(&self, key: Key, modifiers: Modifiers);
    fn key_released(&self, key: Key, modifiers: Modifiers);
}


pub struct Runner {
    app : App,
    window: glfw::Window,
    events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
    glfw: glfw::Glfw,
    frame_rate : f64,
    last_time: std::time::Instant
}


impl Runner {
    pub fn new(app: App) -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));

        let (mut window, events) = glfw.create_window(1280, 720, "Hello this is window", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_key_polling(true);
        window.make_current();

        unsafe {
            gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        }

        app.setup();
        Runner { app: app, window: window, events: events, glfw: glfw, frame_rate: 60.0 as f64,  last_time: std::time::Instant::now() }
    }


    pub fn run(&mut self) {
        while !self.window.should_close() {
            self.glfw.poll_events();
            self.last_time = std::time::Instant::now();
            let millisec_at_fps = std::time::Duration::from_millis((1.0 / self.frame_rate.clone() * 1000.0) as u64);

            self.app.update();
            self.app.draw();

            self.window.swap_buffers();

            for (_, event) in glfw::flush_messages(&self.events) {
                match event {
                    glfw::WindowEvent::Key(key, _, action, modifiers) => {
                        match action {
                            Action::Press => {
                                match key {
                                    Key::Escape => {
                                        self.window.set_should_close(true);
                                    },
                                    _ => {
                                        self.app.key_pressed(key, modifiers);
                                    }
                                }
                            },
                            Action::Release => {
                                match key {
                                    _ => {
                                        self.app.key_released(key, modifiers);
                                    }
                                }
                            },
                            Action::Repeat => {}
                            _ => {}
                        }
                    }
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