#version 330 core

out vec4 color;

in float count_cells;

void main(void) {
    color = vec4((1.0 - count_cells)/5.0, count_cells/2.0, 0.0, 0.01);
}