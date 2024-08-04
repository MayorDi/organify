#version 330 core

layout (location = 0) in vec2 vert_pos;
layout (location = 1) in vec2 tex_pos;

out vec2 st;

uniform vec2 u_resolution;

void main(void) {
    st = tex_pos;
    vec2 uv = vert_pos.xy / u_resolution.xy;
    gl_Position = vec4(uv.xy, 0.0, 1.0);
}