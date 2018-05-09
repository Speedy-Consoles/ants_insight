#version 400

uniform mat3 trafo_matrix;

in vec2 position;

void main() {
    gl_Position = vec4((trafo_matrix * vec3(position, 1.0)).xy, 0.0, 1.0);
}
