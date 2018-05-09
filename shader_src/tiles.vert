#version 400

in vec2 position;
in vec4 color;

out vec4 vg_color;

out vec2 vg_position;

void main() {
    vg_position = position;
    vg_color = color;
}
