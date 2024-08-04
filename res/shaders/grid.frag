#version 330 core

out vec4 color;

uniform bool u_is_empty;

void main(void) {
    if (u_is_empty)
        color = vec4(1.0, 0.0, 0.0, 1.0);
    else
        color = vec4(0.0, 1.0, 0.0, 1.0);
}