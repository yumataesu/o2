#![allow(dead_code)]

mod helper;

use glutin::{ContextWrapper, PossiblyCurrent, dpi::*};
use glutin::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

// use rand::Rng;
// mod base_app;
// mod app;

pub struct WindowSetting {
    title: String,
    is_decoration: bool,
    is_resizable: bool,
    is_always_on_top: bool,
    has_transparent: bool,
    size: PhysicalSize<i32>,
    minimum_size: PhysicalSize<i32>
}

//https://ifritjp.github.io/documents/rust/lifetime/
impl WindowSetting {
    pub fn new() -> Self {
        WindowSetting {
            title: String::from("default title"),
            is_decoration: true,
            is_resizable: false,
            is_always_on_top: false,
            has_transparent: false,
            size: PhysicalSize::new(512, 512),
            minimum_size: PhysicalSize::new(256, 256)
        }
    }
}


pub trait BaseApp {
    fn setup(&self);
    fn update(&self);
    fn draw(&self);
    // fn key_pressed(&self, code);
    // fn key_released(&self);
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
    }


    fn update(&self) {
        println!("update");
    }


    fn draw(&self) {
        println!("draw");
    }
}

#[derive(Debug)]
struct Runner<T: 'static> {
    app : App,
    el: EventLoop<T>,
    windowed_context: ContextWrapper<PossiblyCurrent, glutin::window::Window>,
    frame_rate : f64,
    last_time: std::time::Instant
}


impl Runner<()> {
    pub fn new(app: App) -> Self {
        let el = EventLoop::new();
        let wb = WindowBuilder::new()
                .with_title("My Window")
                .with_decorations(false)
                .with_resizable(false)
                .with_always_on_top(false)
                .with_transparent(true)
                .with_inner_size(PhysicalSize::new(512, 512))
                .with_min_inner_size(PhysicalSize::new(256, 256));

        let windowed_context = ContextBuilder::new()
                    .with_gl(glutin::GlRequest::Latest)
                    .build_windowed(wb, &el).unwrap();
        let windowed_context = unsafe { windowed_context.make_current().unwrap() };

        println!("Pixel format of the window's GL context: {:?}", windowed_context.get_pixel_format());

        let gl_context = windowed_context.context();
        let _gl = gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

        let version = unsafe {
            let data = helper::CStr::from_ptr(gl::GetString(gl::VERSION) as *const _)
                .to_bytes()
                .to_vec();
            String::from_utf8(data).unwrap()
        };
        println!("OpenGL version {}", version);

        app.setup();
        Runner { app : app, el: el, windowed_context: windowed_context, frame_rate: 60.0, last_time: std::time::Instant::now() }
    }


    pub fn run(self) {
        self.el.run(move | event, _, control_flow | {
            match event {
                    Event::WindowEvent{event, ..} =>
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::KeyboardInput{input: KeyboardInput { virtual_keycode: Some(virtual_code), state, .. }, ..} =>
                        match state {
                            ElementState::Pressed => { /*self.app.key_pressed(virtual_code);*/ },
                            ElementState::Released => { /*self.app.key_released(virtual_code);*/ },
                            _ =>()
                        }
                        _ =>()
                    },
                    Event::MainEventsCleared => {
                        //self.app.update();
                        // self.windowed_context.window().request_redraw();
                    },
                    Event::RedrawRequested(_) => {
                        // self.app.draw();
                    },
                    _ => *control_flow = ControlFlow::Poll
                }
        });
    }
            /*            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::KeyboardInput{input: KeyboardInput { virtual_keycode: Some(virtual_code), state, .. }, ..} => 
                        match state {
                            ElementState::Pressed => {
                                match virtual_code {
                                    VirtualKeyCode::Escape => {
                                        *control_flow = ControlFlow::Exit;
                                    },
                                    VirtualKeyCode::Left | VirtualKeyCode::Down | VirtualKeyCode::Right | VirtualKeyCode::Up => {
                                        let p = self.windowed_context.window().outer_position();
                                        let x = p.clone().unwrap().x;
                                        let y = p.clone().unwrap().y;
                                        let mut new = PhysicalPosition::new(0, 0);
                                        let offset = 10;
                                        if virtual_code == VirtualKeyCode::Left {
                                            new = PhysicalPosition::new(x - offset, y);
                                        } else if virtual_code == VirtualKeyCode::Down {
                                            new = PhysicalPosition::new(x, y + offset);
                                        } else if virtual_code == VirtualKeyCode::Right {
                                            new = PhysicalPosition::new(x + offset, y);
                                        } else if virtual_code == VirtualKeyCode::Up {
                                            new = PhysicalPosition::new(x, y - offset);
                                        }
                                        self.windowed_context.window().set_outer_position(new);
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
                    self.last_time = std::time::Instant::now();
                    let millisec_at_fps = std::time::Duration::from_millis((1.0 / self.frame_rate.clone() * 1000.0) as u64);

                    self.windowed_context.window().request_redraw();
    
                    let duration = std::time::Instant::now().duration_since(self.last_time);
                    if duration < millisec_at_fps {
                        std::thread::sleep(millisec_at_fps - duration);
                    }
                },
                Event::RedrawRequested(_) => {
                    unsafe {
                        // gl::ClearColor(n, n, n, 1.0);
                        gl::Clear(gl::COLOR_BUFFER_BIT);
    
                        // gl::UseProgram(program);
                        // gl::BindVertexArray(vao);
                        // gl::DrawArrays(gl::TRIANGLES, 0, 3);
                        // gl::BindVertexArray(0);
                        // gl::UseProgram(0);
                    }
                    self.windowed_context.swap_buffers().unwrap();
                }
                _ => *control_flow = ControlFlow::Poll
            }
    */

}

static NUM : i32 = 111;

fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

const CONFIG : i32 = 12;


fn main() {
    const TEST : i32 = 222;
    let mut window_setting = WindowSetting::new();
    window_setting.has_transparent = false;

    println!("{}", NUM);
    {
        let coerce_num: i32 = 9;
        println!("{}", coerce_static(&coerce_num));
    }

    println!("{}", NUM);
    let x: &'static str = "Hello, world.";

    // x = "fff"

    let r: Option<i32> = Some(123);
    println!("{}", r.unwrap());

    println!("{:?}", Runner::new(App::default()));


    {
    // let el = EventLoop::new();
    // let wb = WindowBuilder::new()
    //             .with_title("My Window")
    //             .with_decorations(false)
    //             .with_resizable(false)
    //             .with_always_on_top(false)
    //             .with_transparent(true)
    //             .with_inner_size(PhysicalSize::new(512, 512))
    //             .with_min_inner_size(PhysicalSize::new(256, 256));

    // let windowed_context = ContextBuilder::new()
    //             .with_gl(glutin::GlRequest::Latest)
    //             .build_windowed(wb, &el).unwrap();
    // let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    // el.run(move | event, _, control_flow | {
    //     match event {
    //         Event::WindowEvent{event, ..} =>
    //         match event {
    //             WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
    //             WindowEvent::KeyboardInput{input: KeyboardInput { virtual_keycode: Some(virtual_code), state, .. }, ..} => 
    //             match state {
    //                 ElementState::Pressed => {
    //                     match virtual_code {
    //                         VirtualKeyCode::Escape => {
    //                             *control_flow = ControlFlow::Exit;
    //                         },
    //                         VirtualKeyCode::Left | VirtualKeyCode::Down | VirtualKeyCode::Right | VirtualKeyCode::Up => {
    //                             let p = windowed_context.window().outer_position();
    //                             let x = p.clone().unwrap().x;
    //                             let y = p.clone().unwrap().y;
    //                             let mut new = PhysicalPosition::new(0, 0);
    //                             let offset = 10;
    //                             if virtual_code == VirtualKeyCode::Left {
    //                                 new = PhysicalPosition::new(x - offset, y);
    //                             } else if virtual_code == VirtualKeyCode::Down {
    //                                 new = PhysicalPosition::new(x, y + offset);
    //                             } else if virtual_code == VirtualKeyCode::Right {
    //                                 new = PhysicalPosition::new(x + offset, y);
    //                             } else if virtual_code == VirtualKeyCode::Up {
    //                                 new = PhysicalPosition::new(x, y - offset);
    //                             }
    //                             windowed_context.window().set_outer_position(new);
    //                         },
    //                         _ => ()
    //                     }
    //                 },
    //                 ElementState::Released => {

    //                 }
    //             }
    //             _ =>()
    //     }
    //     Event::MainEventsCleared => {
    //     },
    //     Event::RedrawRequested(_) => {
    //     }
    //     _ => *control_flow = ControlFlow::Poll
    //     }
    // });
}
}

    // Runner{ app : &app::App::default() };
    //let mut window_setting : windowSetting;
    //window_setting.fewafewafewa = fewafewa;
    //Runner(app::App::new(window_setting)).run()

    // let mydata_on_stack = MyData::new();
    // let mut mydata_on_heap : Box<MyData> = Box::new(MyData::new());
    
    // mydata_on_heap.test();
    // MyData::Test();

    // let mydata_on_heap2 = mydata_on_heap.clone();
    // println!("{:#?}", mydata_on_heap);
    // println!("{:#?}", mydata_on_heap2);

    // //String と Stringの連結について
    // helper::string::join_string_to_string();
    // helper::string::join_strref_to_strref();

    // let a = "abc";
    // let b = "ab";
    // if helper::string::starts_with(&a, &b) {
    //     println!("has words");
    // }
    // else {
    //     println!("no words found");
    // }

    // if helper::string::is_phone_number("000-1234-7362") {
    //     println!("is_phone_number");
    // }
    // else {
    //     println!("not phone number");
    // }

    // let mut app = base_app::App::new();
    // app.with_title("My Rust Window");

    // let mut a : base_app::App<()> = app::App::new();
    // app.with_resizable(false).with_always_on_top(false);
    //     
    //     // .with_always_on_top(false)
    //     .with_transparent(true)
    //     .with_inner_size(PhysicalSize::new(512, 512))
    //     .with_min_inner_size(PhysicalSize::new(256, 256));

    //app.create_gl_context();

    // let fps : f64 = 60.0;
    // let el = EventLoop::new();
    // let wb = WindowBuilder::new()
    //             .with_title("My Window")
    //             .with_decorations(false)
    //             .with_resizable(false)
    //             .with_always_on_top(false)
    //             .with_transparent(true)
    //             .with_inner_size(PhysicalSize::new(512, 512))
    //             .with_min_inner_size(PhysicalSize::new(256, 256));

    // let windowed_context = ContextBuilder::new()
    //             .with_gl(glutin::GlRequest::Latest)
    //             .build_windowed(wb, &el).unwrap();
    // let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    // println!("Pixel format of the window's GL context: {:?}", windowed_context.get_pixel_format());

    // let gl_context = windowed_context.context();
    // let _gl = gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    // let version = unsafe {
    //     let data = helper::CStr::from_ptr(gl::GetString(gl::VERSION) as *const _)
    //         .to_bytes()
    //         .to_vec();
    //     String::from_utf8(data).unwrap()
    // };

    // println!("OpenGL version {}", version);


    // let mut last = std::time::Instant::now();
    // let millisec_at_fps = std::time::Duration::from_millis((1.0 / fps * 1000.0) as u64);
    // let mut rng = rand::thread_rng();
    // let mut n : f32 = 0.0;
    // println!("{}", &helper::util::type_of(n));


    /*
    static VERTEX_DATA: [f32; 15] = [
        -0.5, -0.5,  1.0,  0.0,  0.0,
         0.0,  0.5,  0.0,  1.0,  0.0,
         0.5, -0.5,  0.0,  0.0,  1.0,
    ];

    let vs_src = fs::read_to_string("data/shader/shader.vert").expect("Something went wrong reading the file");
    let fs_src = fs::read_to_string("data/shader/shader.frag").expect("Something went wrong reading the file");

    let program = unsafe { gl::CreateProgram() };
    let mut vao = unsafe { std::mem::zeroed() };
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
        
        let mut vb = std::mem::zeroed();
        gl::GenBuffers(1, &mut vb);
        gl::BindBuffer(gl::ARRAY_BUFFER, vb);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTEX_DATA.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            VERTEX_DATA.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );
        
        if gl::BindVertexArray::is_loaded() {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
        }

        let pos_attrib = gl::GetAttribLocation(program, b"position\0".as_ptr() as *const _);
        let color_attrib = gl::GetAttribLocation(program, b"color\0".as_ptr() as *const _);
        gl::VertexAttribPointer(
            pos_attrib as gl::types::GLuint,
            2,
            gl::FLOAT,
            0,
            5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
            std::ptr::null(),
        );
        gl::VertexAttribPointer(
            color_attrib as gl::types::GLuint,
            3,
            gl::FLOAT,
            0,
            5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
            (2 * std::mem::size_of::<f32>()) as *const () as *const _,
        );
        gl::EnableVertexAttribArray(pos_attrib as gl::types::GLuint);
        gl::EnableVertexAttribArray(color_attrib as gl::types::GLuint);
        gl::BindVertexArray(0);
    }*/


/* 
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
                                    let offset = 10;
                                    if virtual_code == VirtualKeyCode::Left {
                                        new = PhysicalPosition::new(x - offset, y);
                                    } else if virtual_code == VirtualKeyCode::Down {
                                        new = PhysicalPosition::new(x, y + offset);
                                    } else if virtual_code == VirtualKeyCode::Right {
                                        new = PhysicalPosition::new(x + offset, y);
                                    } else if virtual_code == VirtualKeyCode::Up {
                                        new = PhysicalPosition::new(x, y - offset);
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
                n *= 0.05;

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

                    // gl::UseProgram(program);
                    // gl::BindVertexArray(vao);
                    // gl::DrawArrays(gl::TRIANGLES, 0, 3);
                    // gl::BindVertexArray(0);
                    // gl::UseProgram(0);
                }
                windowed_context.swap_buffers().unwrap();
            }
            _ => *control_flow = ControlFlow::Poll
        }

    });*/


