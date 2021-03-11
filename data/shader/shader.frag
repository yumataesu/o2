#version 450

in vec4 v_color;
in vec2 v_texcoord;

uniform sampler2D u_src;

layout (location = 0) out vec4 FragColor;

void main() {
    FragColor = vec4(v_texcoord, 0, 1);
    FragColor = texture(u_src, v_texcoord);
}