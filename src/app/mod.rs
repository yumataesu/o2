use crate::framework::{self};
use imgui_glfw_rs::glfw::{Key, Modifiers, MouseButton};
use imgui_glfw_rs::imgui;
use framework::{Load, Allocate, New};

use rand::distributions::*;


#[derive(Default)]
pub struct App {
    val: f32,
    shader: framework::Shader,
    vao: framework::Vao,
    ebo: framework::BufferObject,
    position_vbo: framework::BufferObject,
    color_vbo: framework::BufferObject,
    texcoord_vbo: framework::BufferObject,
    positions: Vec<glam::Vec3>,
    vel: Vec<glam::Vec3>,
    acc: Vec<glam::Vec3>,
    colors: Vec<glam::Vec4>,
    texcoords: Vec<glam::Vec2>,
    indices: Vec<u32>,
    num: usize,
    center: glam::Vec3,
    tex: framework::Texture,
    fbo: framework::FrameBuffer,
    cam_pos: glam::Vec3,
    cam_lookat: glam::Vec3,
    cam_fov: f32,
    // event_system: EventSystem,
    // score: GameResult<WrappedScore>
}

impl framework::BaseApp for App {

    fn setup(&mut self) {
        self.shader = framework::Shader::new();
        self.shader.load("data/shader/shader");
        
        self.fbo = framework::FrameBuffer::new();
        self.fbo.allocate((1920, 1080, gl::RGBA as i32, gl::COLOR_ATTACHMENT0));

        self.num = 4;
        let prange = rand::distributions::Uniform::new(-1.0f32, 1.0);
        let crange = rand::distributions::Uniform::new(0.0f32, 1.0);
        let mut rng = rand::thread_rng();

        self.positions = Vec::with_capacity(self.num);
        self.colors = Vec::with_capacity(self.num);
        for i in 0..self.num {
            //self.positions.push(glam::Vec3::new(prange.sample(&mut rng), prange.sample(&mut rng), 0.0));
            //self.vel.push(glam::Vec3::new(0.0, 0.0, 0.0));
            //self.acc.push(glam::Vec3::new(0.0, 0.0, 0.0));
            self.colors.push(glam::Vec4::new(crange.sample(&mut rng), crange.sample(&mut rng), crange.sample(&mut rng), 1.0));
        }

        let w = 1.0 / 1.0;
        let h = 1.0 / 1.0;

        self.positions.push(glam::Vec3::new(-w, -h, 0.0));
        self.positions.push(glam::Vec3::new(w, -h, 0.0));
        self.positions.push(glam::Vec3::new(w, h, 0.0));
        self.positions.push(glam::Vec3::new(-w, h, 0.0));

        self.texcoords.push(glam::Vec2::new(0.0, 1.0));
        self.texcoords.push(glam::Vec2::new(1.0, 1.0));
        self.texcoords.push(glam::Vec2::new(1.0, 0.0));
        self.texcoords.push(glam::Vec2::new(0.0, 0.0));

        self.indices.push(0);
        self.indices.push(1);
        self.indices.push(2);
        self.indices.push(0);
        self.indices.push(3);
        self.indices.push(2);

        self.cam_pos = glam::Vec3::new(0.0,0.0,1.0);
        self.cam_lookat = glam::Vec3::new(0.0,0.0,0.0);
        self.cam_fov = 60.0;

        self.tex = framework::Texture::new("data/test.jpeg");

        self.position_vbo = framework::BufferObject::new();
        self.position_vbo.allocate((framework::VertexAttribute::Position, &self.positions));

        self.color_vbo = framework::BufferObject::new();
        self.color_vbo.allocate((framework::VertexAttribute::Color, &self.colors));

        self.texcoord_vbo = framework::BufferObject::new();
        self.texcoord_vbo.allocate((framework::VertexAttribute::Texcoord, &self.texcoords));

        self.ebo = framework::BufferObject::new();
        self.ebo.allocate((framework::VertexAttribute::Index, &self.indices));

        self.vao = framework::Vao::new();
        self.vao.set_vbo(&self.position_vbo);
        self.vao.set_vbo(&self.color_vbo);
        self.vao.set_vbo(&self.texcoord_vbo);
        self.vao.set_vbo(&self.ebo);

        self.fbo.clear();

        // self.event_system = EventSystem::new();
        // let mouse_event = MouseEventArgs::new().unwrap();
        // self.event_system.add_observer(mouse_event.clone());
    }


    fn update(&mut self) {
        // for i in 0..self.num {
        //     self.acc[i] = glam::Vec3::new(0.0,0.0,0.0);
        //     self.acc[i] = self.center - self.positions[i];
        //     self.acc[i] = self.acc[i].normalize()* 0.1;
        //     self.vel[i] += self.acc[i] * 0.001;
        //     self.positions[i] += self.vel[i];
        // }
        //self.position_vbo.update(&self.positions);
    }


    fn draw(&mut self) {
        //view mat
        // self.cam_pos = glam::vec3(0.0, 0.1, 0.2);
        // glam::mat4(x_axis, y_axis, z_axis, w_axis)
        let f = (self.cam_lookat - self.cam_pos).normalize();
        let s = f.cross(glam::vec3(0.0,1.0,0.0)).normalize();
        let u = s.cross(f).normalize();

        let vx = glam::vec4(s.x, s.y, s.z, -s.dot(self.cam_pos));
        let vy = glam::vec4(u.x, u.y, u.z, -u.dot(self.cam_pos));
        let vz = glam::vec4(-f.x, -f.y, -f.z, f.dot(self.cam_pos));
        let vw = glam::vec4(0.0,0.0,0.0,1.0);
        let mut view = glam::mat4(vx, vy, vz, vw);
        view = view.transpose();
        // println!("{}", view);
        // println!("==============");

        //prj mat
        let near = 1.0;
        let far = 1000.0;
        let aspect = 1920.0 / 1080.0;
        let tan_half = ((self.cam_fov * (std::f32::consts::PI / 180.0)) / 2.0).tan();
        let px = glam::vec4(1.0 / (aspect * tan_half), 0.0, 0.0, 0.0);
        let py = glam::vec4(0.0, 1.0 / tan_half, 0.0, 0.0);
        let pz = glam::vec4(0.0, 0.0, -(far + near) / (far - near), -1.0);
        let pw = glam::vec4(0.0, 0.0, -(2.0 * far * near) / (far - near), 0.0);
        let mut projection = glam::mat4(px, py, pz, pw);
        projection = projection.transpose();
        //println!("{}", projection.transpose());
        let projection = glam::Mat4::perspective_rh(self.cam_fov * (std::f32::consts::PI / 180.0), aspect, near, far);
        //let mut view = glam::Mat4::look_at_rh(self.cam_pos, self.cam_lookat, glam::vec3(0.0,1.0,0.0));
        //println!("{}", projection);
        //println!("==============");

        let model = glam::Mat4::IDENTITY;
        self.fbo.begin();
        framework::gl_utils::clear_color(0.1, 0.1, 0.1, 0.1);
        framework::gl_utils::clear();
        self.shader.begin();
        self.shader.uniform_texture("u_src", self.tex.get());
        self.shader.uniform_mat4("projection", &projection);
        self.shader.uniform_mat4("view", &view);
        self.shader.uniform_mat4("model", &model);
        self.vao.draw_elements(gl::TRIANGLES);
        self.shader.end();
        self.fbo.end();

        self.fbo.draw(0);
    }


    fn draw_gui(&mut self, ui: &imgui::Ui) {
        // ui.show_demo_window(&mut true);
        // let win = ui.window(imgui::im_str!("title"));
        ui.slider_float(imgui::im_str!("Cam Fov"), &mut self.cam_fov, 20.0, 100.0).build();
        ui.slider_float(imgui::im_str!("Cam Pos X"), &mut self.cam_pos.x, -10.0, 10.0).build();
        ui.slider_float(imgui::im_str!("Cam Pos y"), &mut self.cam_pos.y, -10.0, 10.0).build();
        ui.slider_float(imgui::im_str!("Cam Pos z"), &mut self.cam_pos.z, -10.0, 10.0).build();
        // ui.slider_float3(imgui::im_str!("camera position"), &mut self.cam_pos.as_ref().as_ptr(), -1.0, 1.0);
    }


    fn key_pressed(&mut self, key: Key, modifiers: Modifiers) {
        // println!("key_pressed {:?}", key);
        //self.shader = framework::Shader::new();
        //self.shader.load("data/shader/shader");
    }

    fn key_released(&mut self, key: Key, modifiers: Modifiers) {
        // println!("key_released {:?}", key);
    }

    fn mouse_pressed(&mut self, button: MouseButton) {
        // println!("mouse_pressed {:?}", button);
        println!("mouse_pressed");
        // self.event_system.notify(Event::MouseEvent);
    }

    fn mouse_released(&mut self, button: MouseButton) {
        // println!("mouse_released {:?}", button);
    }

    fn file_dropped(&mut self, paths: Vec<std::path::PathBuf>) {
        // println!("file_dropped {:?}", paths);
    }

    fn cursor_moved(&mut self, x: f32, y: f32) {
        // println!("cursor_moved {}, {}", x, y);
        // self.center = glam::Vec3::new(x / 1920.0f32 * 2.0 - 1.0, 1.0 - (y / 1080.0f32 * 2.0 - 1.0), 0.0);
    }
}