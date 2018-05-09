#version 400

uniform mat3 trafo_matrix;

const vec2 offsets[4] = vec2[4](
    vec2 (0.0, 0.0),
    vec2 (1.0, 0.0),
    vec2 (1.0, 1.0),
    vec2 (0.0, 1.0)
);

layout(points) in;
layout(triangle_strip, max_vertices = 6) out;

in vec2 vg_position[];
in vec4 vg_color[];

out vec4 gf_color;

void main() {
    vec2[4] vertices = vec2[4] (
        trafo_matrix * vec3(vg_position[0] + offsets[0], 1.0),
        trafo_matrix * vec3(vg_position[0] + offsets[1], 1.0),
        trafo_matrix * vec3(vg_position[0] + offsets[2], 1.0),
        trafo_matrix * vec3(vg_position[0] + offsets[3], 1.0)
    );
    gf_color = vg_color[0];


    gl_Position = vec4(vertices[0].xy, 0.0, 1.0);
    EmitVertex();

    gl_Position = vec4(vertices[1].xy, 0.0, 1.0);
    EmitVertex();

    gl_Position = vec4(vertices[2].xy, 0.0, 1.0);
    EmitVertex();

    gl_Position = vec4(vertices[0].xy, 0.0, 1.0);
    EmitVertex();

    gl_Position = vec4(vertices[2].xy, 0.0, 1.0);
    EmitVertex();

    gl_Position = vec4(vertices[3].xy, 0.0, 1.0);
    EmitVertex();
}