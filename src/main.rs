mod helper;

use glutin::dpi::*;
use glutin::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use rand::Rng;
extern crate gl;
extern crate libc;

fn main() {
    let fps : f64 = 60.0;
    let el = glutin::event_loop::EventLoop::new();
    let wb = WindowBuilder::new()
                .with_title("My Window!")
                .with_decorations(true)
                .with_resizable(false)
                .with_always_on_top(false)
                .with_inner_size(PhysicalSize::new(720, 1280))
                .with_min_inner_size(PhysicalSize::new(256, 256));

    let windowed_context = ContextBuilder::new().build_windowed(wb, &el).unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    // println!("Pixel format of the window's GL context: {:?}", windowed_context.get_pixel_format());

    let gl_context = windowed_context.context();
    let gl = gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    let version = unsafe {
        let data = helper::CStr::from_ptr(gl::GetString(gl::VERSION) as *const _)
            .to_bytes()
            .to_vec();
        String::from_utf8(data).unwrap()
    };

    println!("OpenGL version {}", version);

    let mut n : f32 = 0.0;

    let mut last = std::time::Instant::now();
    let millisec_at_fps = std::time::Duration::from_millis((1.0 / fps * 1000.0) as u64);
    let mut rng = rand::thread_rng();

    el.run(move | event, _, control_flow | {
        match event {
                Event::WindowEvent{event, ..} =>
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput{input: KeyboardInput { virtual_keycode: Some(virtual_code), state, .. }, ..} => 
                    match state {
                        ElementState::Pressed => {
                            match virtual_code {
                                VirtualKeyCode::Escape => {
                                    *control_flow = ControlFlow::Exit;
                                },
                                VirtualKeyCode::Left | VirtualKeyCode::Down | VirtualKeyCode::Right | VirtualKeyCode::Up => {
                                    let p = windowed_context.window().outer_position();
                                    let x = p.clone().unwrap().x;
                                    let y = p.clone().unwrap().y;
                                    let mut new = PhysicalPosition::new(0, 0);

                                    if virtual_code == VirtualKeyCode::Left {
                                        new = PhysicalPosition::new(x - 1, y);
                                    } else if virtual_code == VirtualKeyCode::Down {
                                        new = PhysicalPosition::new(x, y + 1);
                                    } else if virtual_code == VirtualKeyCode::Right {
                                        new = PhysicalPosition::new(x + 1, y);
                                    } else if virtual_code == VirtualKeyCode::Up {
                                        new = PhysicalPosition::new(x, y - 1);
                                    }
                                    windowed_context.window().set_outer_position(new);
                                },
                                _ => ()
                            }
                        },
                        ElementState::Released => {

                        }
                    }
                    _ =>()
            }
            Event::MainEventsCleared => {
                last = std::time::Instant::now();
                n = rng.gen();
                n *= 0.1;
                
                windowed_context.window().request_redraw();

                let duration = std::time::Instant::now().duration_since(last);
                if duration < millisec_at_fps {
                    std::thread::sleep(millisec_at_fps - duration);
                }
            },
            Event::RedrawRequested(_) => {
                unsafe {
                    gl::ClearColor(n, n, n, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }
                windowed_context.swap_buffers().unwrap();
            }
            _ => *control_flow = ControlFlow::Poll
        }

    });

}
