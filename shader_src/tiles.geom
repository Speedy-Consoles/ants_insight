#version 400

uniform mat4 trafo_matrix;

const vec3 offsets[4] = vec3[4](
    vec3 (0.0, 0.0, 0.0),
    vec3 (1.0, 0.0, 0.0),
    vec3 (1.0, 1.0, 0.0),
    vec3 (0.0, 1.0, 0.0)
);

layout(points) in;
layout(triangle_strip, max_vertices = 4) out;

in vec3 vg_position[];
in vec4 vg_color[];
in float vg_radius2[];

out vec4 gf_color;
out vec3 gf_position;
out float gf_radius2;
out vec3 gf_center;

void main() {
    vec3[4] vertices = vec3[4] (
        vg_position[0] + offsets[0],
        vg_position[0] + offsets[1],
        vg_position[0] + offsets[2],
        vg_position[0] + offsets[3]
    );
    gf_color = vg_color[0];
    gf_radius2 = vg_radius2[0];
    gf_center = vg_position[0] + vec3(0.5, 0.5, 0.0);

    gf_position = vertices[1];
    gl_Position = trafo_matrix * vec4(vertices[1], 1.0);
    EmitVertex();

    gf_position = vertices[0];
    gl_Position = trafo_matrix * vec4(vertices[0], 1.0);
    EmitVertex();

    gf_position = vertices[2];
    gl_Position = trafo_matrix * vec4(vertices[2], 1.0);
    EmitVertex();

    gf_position = vertices[3];
    gl_Position = trafo_matrix * vec4(vertices[3], 1.0);
    EmitVertex();
}