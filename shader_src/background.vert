#version 400

uniform mat4 trafo_matrix;

in vec3 position;

void main() {
    gl_Position = vec4(trafo_matrix * vec4(position, 1.0));
}
