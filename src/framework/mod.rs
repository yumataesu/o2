use gl;
use glfw::*;

use imgui::{FontConfig, FontGlyphRanges, FontSource, Ui};

use std::{ffi::CStr, slice::Windows};
use std::os::raw::c_void;

pub mod util;
pub mod settings;
use crate::app::App;


pub struct GlfwPlatform {
    hidpi_mode: ActiveHiDpiMode,
    hidpi_factor: f64,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum ActiveHiDpiMode {
    Default,
    Rounded,
    Locked,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum HiDpiMode {
    Default,
    Rounded,
    Locked(f64),
}

struct Clipboard {
    window_ptr: *mut glfw::ffi::GLFWwindow,
}

impl imgui::ClipboardBackend for Clipboard {
    fn set(&mut self, s: &imgui::ImStr) {
        unsafe {
            glfw::ffi::glfwSetClipboardString(self.window_ptr, s.as_ptr());
        }
    }
    fn get(&mut self) -> std::option::Option<imgui::ImString> {
        unsafe {
            let s = glfw::ffi::glfwGetClipboardString(self.window_ptr);
            let s = std::ffi::CStr::from_ptr(s);
            let bytes = s.to_bytes();
            if !bytes.is_empty() {
                let v = String::from_utf8_lossy(bytes);
                Some(imgui::ImString::new(v))
            } else {
                None
            }
        }
    }
}

impl HiDpiMode {
    fn apply(&self, hidpi_factor: f64) -> (ActiveHiDpiMode, f64) {
        match *self {
            HiDpiMode::Default => (ActiveHiDpiMode::Default, hidpi_factor),
            HiDpiMode::Rounded => (ActiveHiDpiMode::Rounded, hidpi_factor.round()),
            HiDpiMode::Locked(value) => (ActiveHiDpiMode::Locked, value),
        }
    }
}


pub trait BaseApp {
    fn setup(&self);
    fn update(&self);
    fn draw(&self);
    fn key_pressed(&self, key: Key, modifiers: Modifiers);
    fn key_released(&self, key: Key, modifiers: Modifiers);
    fn mouse_pressed(&self, button: MouseButton);
    fn mouse_released(&self, button: MouseButton);
    fn cursor_moved(&self, x: f64, y: f64);
    fn file_dropped(&self, paths: Vec<std::path::PathBuf>);
}


pub struct Runner {
    app : App,
    window: glfw::Window,
    events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
    glfw: glfw::Glfw,
    window_settings: settings::WindowSettings,
    frame_rate : f64,
    last_time: std::time::Instant,
    imgui: imgui::Context
}


impl Runner {
    pub fn new(app: App, ws: settings::WindowSettings) -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(ws.gl_version.0.clone(), ws.gl_version.1.clone()));

        let (mut window, events) = glfw.create_window(ws.window_size.0.clone(), ws.window_size.1.clone(), &ws.title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_key_polling(true);
        window.set_mouse_button_polling(true);
        window.set_scroll_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_drag_and_drop_polling(true);
        window.make_current();

        unsafe {
            gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        }
        let mut  hidpi_mode: ActiveHiDpiMode;
        let mut hidpi_factor: f64;
        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);

        let mut io_mut = imgui.io_mut();
        io_mut.key_map[imgui::Key::Tab as usize] = Key::Tab as u32;
        io_mut.key_map[imgui::Key::LeftArrow as usize] = Key::Left as u32;
        io_mut.key_map[imgui::Key::RightArrow as usize] = Key::Right as u32;
        io_mut.key_map[imgui::Key::UpArrow as usize] = Key::Up as u32;
        io_mut.key_map[imgui::Key::DownArrow as usize] = Key::Down as u32;
        io_mut.key_map[imgui::Key::PageUp as usize] = Key::PageUp as u32;
        io_mut.key_map[imgui::Key::PageDown as usize] = Key::PageDown as u32;
        io_mut.key_map[imgui::Key::Home as usize] = Key::Home as u32;
        io_mut.key_map[imgui::Key::End as usize] = Key::End as u32;
        io_mut.key_map[imgui::Key::Insert as usize] = Key::Insert as u32;
        io_mut.key_map[imgui::Key::Delete as usize] = Key::Delete as u32;
        io_mut.key_map[imgui::Key::Backspace as usize] = Key::Backspace as u32;
        io_mut.key_map[imgui::Key::Space as usize] = Key::Space as u32;
        io_mut.key_map[imgui::Key::Enter as usize] = Key::Enter as u32;
        io_mut.key_map[imgui::Key::Escape as usize] = Key::Escape as u32;
        io_mut.key_map[imgui::Key::A as usize] = Key::A as u32;
        io_mut.key_map[imgui::Key::C as usize] = Key::C as u32;
        io_mut.key_map[imgui::Key::V as usize] = Key::V as u32;
        io_mut.key_map[imgui::Key::X as usize] = Key::X as u32;
        io_mut.key_map[imgui::Key::Y as usize] = Key::Y as u32;
        io_mut.key_map[imgui::Key::Z as usize] = Key::Z as u32;
        // let renderer = Renderer::new(imgui, |s| window.get_proc_address(s) as _);
        // let (scale_factor_x, _scale_factor_y) = window.get_content_scale();
        // let (hidpi_mode, hidpi_factor) = hidpi_mode.apply(scale_factor_x as _);
        // self.hidpi_mode = hidpi_mode;
        // self.hidpi_factor = hidpi_factor;
        // io_mut.display_framebuffer_scale = [hidpi_factor as f32, hidpi_factor as f32];
        let (width, height) = window.get_size();
        io_mut.display_size = [width as f32, height as f32];

        unsafe {
            let window_ptr = window.window_ptr();
            imgui.set_clipboard_backend(Box::new(Clipboard { window_ptr }));
        }

        app.setup();
        Runner { 
            app: app, 
            window: window, 
            events: events, 
            glfw: glfw,
            window_settings: ws,
            frame_rate: 60.0 as f64, 
            last_time: std::time::Instant::now(),
            imgui: imgui
        }
    }


    pub fn run(&mut self) {
        while !self.window.should_close() {
            self.glfw.poll_events();
            self.last_time = std::time::Instant::now();
            let millisec_at_fps = std::time::Duration::from_millis((1.0 / self.frame_rate.clone() * 1000.0) as u64);

            self.app.update();
            self.app.draw();


            let mut ui = &self.imgui.frame();
            let mut win = imgui::Window::new(imgui::im_str!("Hello world"))
                .size([300.0, 110.0], imgui::Condition::FirstUseEver);
                    if let Some(window) = win.begin(ui) {
                        window.end(ui);
                    }

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
                    },
                    glfw::WindowEvent::MouseButton(button, action, modifiers) => {
                        match action {
                            Action::Press => {
                                self.app.mouse_pressed(button);
                            },
                            Action::Release => {
                                self.app.mouse_released(button);
                            },
                            Action::Repeat => {}
                        }
                    },
                    glfw::WindowEvent::CursorPos(x, y) => {
                        self.app.cursor_moved(x, y);
                    },
                    glfw::WindowEvent::FileDrop(paths) => {
                        self.app.file_dropped(paths);
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