#version 450

in vec4 v_color;
in vec2 v_texcoord;

uniform sampler2D u_src;

layout (location = 0) out vec4 FragColor;

void main() {
    vec4 result = texture(u_src, v_texcoord);
    FragColor = result;
    // FragColor = vec4(v_texcoord, 1, 1);
}