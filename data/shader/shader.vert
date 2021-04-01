#version 450

layout (location = 0) in vec3 position;
layout (location = 1) in vec4 color;
layout (location = 2) in vec2 texcoord;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;

out vec4 v_color;
out vec2 v_texcoord;

void main() {
    gl_Position = projection * view * model * vec4(position, 1.0);
    // gl_Position = vec4(position, 1.0);

    v_color = color;
    v_texcoord = texcoord;
}
