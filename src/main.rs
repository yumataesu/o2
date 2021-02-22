extern crate gl;
extern crate glfw;

use glfw::{Action, Context, Key, Modifiers};

pub trait BaseApp {
    fn setup(&self);
    fn update(&self);
    fn draw(&self);
    fn key_pressed(&self, key: Key, modifiers: Modifiers);
    fn key_released(&self, key: Key, modifiers: Modifiers);
}

#[derive(Debug, Default)]
pub struct App {
    number: i32
}

impl App {
    pub fn new() -> Self {
        App{number: 123}
    }
}

impl BaseApp for App {

    fn setup(&self) {
        println!("setup");

        let vs_src = std::fs::read_to_string("data/shader/shader.vert").expect("Something went wrong reading the file");
        let fs_src = std::fs::read_to_string("data/shader/shader.frag").expect("Something went wrong reading the file");
        let program = unsafe { gl::CreateProgram() };
        unsafe {
            let vs = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vs, 1, [vs_src.as_bytes().as_ptr() as *const _].as_ptr(), std::ptr::null());
            // gl::ShaderSource(vs, 1, [VS_SRC.as_ptr() as *const _].as_ptr(), std::ptr::null());
            gl::CompileShader(vs);

            let fs = gl::CreateShader(gl::FRAGMENT_SHADER);
            // gl::ShaderSource(fs, 1, [FS_SRC.as_ptr() as *const _].as_ptr(), std::ptr::null());
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


struct Runner {
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
            gl::ClearColor(0.7, 0.7, 0.7, 1.0);
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

fn main() {
    Runner::new(App::new()).run();
}

