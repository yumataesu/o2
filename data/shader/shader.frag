#version 450

in vec3 v_color;

layout (location = 0) out vec4 FragColor;

void main() {
    FragColor = vec4(v_color, 1.0);
}