pub mod Utils {
    pub fn clear_color(r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
        }
    }
    
    pub fn clear() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}


pub const VS_QUAD: &[u8] = b"
#version 450

layout (location = 0) in vec3 position;
layout (location = 2) in vec2 texcoord;

out vec4 v_color;
out vec2 v_texcoord;

void main() {
    gl_Position = vec4(position, 1.0);
    v_texcoord = texcoord;
}
\0";

pub const FS_QUAD: &[u8] = b"
#version 450

in vec4 v_color;
in vec2 v_texcoord;

uniform sampler2D u_src;

layout (location = 0) out vec4 FragColor;

void main() {
    vec4 result = texture(u_src, v_texcoord);
    FragColor = result;
}
\0";

