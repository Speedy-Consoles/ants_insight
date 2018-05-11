#version 400

uniform mat4 trafo_matrix;

layout(points) in;
layout(line_strip, max_vertices = 2) out;

in vec3 vg_start[];
in vec3 vg_end[];
in vec4 vg_color[];

out vec4 gf_color;

void main() {
    gf_color = vg_color[0];

    gl_Position = trafo_matrix * vec4(vg_start[0], 1.0);
    EmitVertex();

    gl_Position = trafo_matrix * vec4(vg_end[0], 1.0);
    EmitVertex();
}