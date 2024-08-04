#version 330 core

in vec2 vert_pos;
uniform vec2 u_resolution;

void main(void) {
    vec2 uv = vert_pos.xy / u_resolution.xy;
    gl_Position = vec4(uv.xy, 0.0, 1.0);
}