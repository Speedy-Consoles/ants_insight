#version 400

in vec3 start;
in vec3 end;
in vec4 color;

out vec3 vg_start;
out vec3 vg_end;
out vec4 vg_color;

void main() {
    vg_start = start;
    vg_end = end;
    vg_color = color;
}
