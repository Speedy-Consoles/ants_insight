#version 400

in vec3 position;
in vec4 color;
in float radius2;

out vec4 vg_color;
out vec3 vg_position;
out float vg_radius2;

void main() {
    vg_position = position;
    vg_color = color;
    vg_radius2 = radius2;
}
