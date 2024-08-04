#version 330 core

out vec4 color;

in vec2 vertex_position;

uniform float u_radius;

void main(void) {
    float d = length(vertex_position.xy);

    if(d < u_radius - 10.0) {
        color = vec4(0.16, 0.18, 0.19, 1.0);
    } else if(d < u_radius) {
        color = vec4(0.26, 0.28, 0.31, 1.0);
    } else {
        color = vec4(0.0);
    }

}