#version 400

in vec2 position;
in vec4 color;

out vec2 vg_position;
out vec4 vg_color;

void main() {
    vg_position = position;
    vg_color = color;
}
