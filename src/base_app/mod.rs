use glutin::dpi::*;
use glutin::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
pub use std::ffi::CStr;


#[derive(Debug)]
pub struct App<T: 'static> {
    event: EventLoop<T>,
    wb: WindowBuilder,
    frame_rate : i32
}

impl App<()> {
    pub fn new() -> Self {
        App {event: EventLoop::new(), wb : WindowBuilder::new(), frame_rate: 60}
    }

    pub fn create_gl_context(self) {
        let windowed_context = ContextBuilder::new()
            .with_gl(glutin::GlRequest::Latest)
            .build_windowed(self.wb, &self.event).unwrap();
        let windowed_context = unsafe { windowed_context.make_current().unwrap() };

        println!("Pixel format of the window's GL context: {:?}", windowed_context.get_pixel_format());

        let gl_context = windowed_context.context();
        let _gl = gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

        let version = unsafe {
            let data = CStr::from_ptr(gl::GetString(gl::VERSION) as *const _)
                .to_bytes()
                .to_vec();
            String::from_utf8(data).unwrap()
        };
    }

    pub fn run() {

    }

    #[inline]
    pub fn with_title<T: Into<String>>(&mut self, title: T) -> &WindowBuilder {
        &self.wb
    }

    #[inline]
    pub fn with_inner_size<S: Into<Size>>(&mut self, size: S) -> &WindowBuilder {
        self.wb.window.inner_size = Some(size.into());
        &self.wb
    }

    #[inline]
    pub fn with_min_inner_size<S: Into<Size>>(&mut self, min_size: S) -> &WindowBuilder {
        self.wb.window.min_inner_size = Some(min_size.into());
        &self.wb
    }

    #[inline]
    pub fn with_decorations(&mut self, decorations: bool) -> &WindowBuilder {
        self.wb.window.decorations = decorations;
        &self.wb
    }
    
    #[inline]
    pub fn with_resizable(&mut self, resizable: bool) -> &WindowBuilder {
        self.wb.window.resizable = resizable;
        &self.wb
    }

    #[inline]
    pub fn with_transparent(&mut self, transparent: bool) -> &WindowBuilder {
        self.wb.window.transparent = transparent;
        &self.wb
    }

    // #[inline]
    // pub fn with_always_on_top(&mut self, always_on_top: bool) -> &WindowBuilder {
    //     &self.wb.with_always_on_top(always_on_top);
    //     &self.wb
    // }
}
