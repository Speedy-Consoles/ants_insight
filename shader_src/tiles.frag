#version 400

in vec4 gf_color;
in vec3 gf_position;
in float gf_radius2;
in vec3 gf_center;

out vec4 f_color;

void main() {
    f_color = gf_color;
    vec2 delta = gf_position.xy - gf_center.xy;
    float d2 = delta.x * delta.x + delta.y * delta.y;
    if (d2 > gf_radius2) {
        discard;
    }
}