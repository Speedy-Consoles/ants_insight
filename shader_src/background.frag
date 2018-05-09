#version 400

uniform vec3 background_color;

out vec4 f_color;

void main() {
    f_color = vec4(background_color, 1.0);
}