#version 330 core

layout (location = 0) in vec2 vert_pos;
layout (location = 1) in vec2 tex_pos;

struct Camera {
    vec2 position;
    float scale;
};

out vec2 st;

uniform vec2 u_resolution;
uniform Camera u_camera;

void main(void) {
    st = tex_pos;
    vec2 uv = vert_pos.xy / u_resolution.xy;
    vec4 n_cam_pos = vec4(u_camera.position.xy / u_resolution.xy, 0.0, 1.0);

    mat4 transform_matrix = mat4(
        1.0,            0.0,            0.0, 0.0,
        0.0,            1.0,            0.0, 0.0,
        0.0,            0.0,            1.0, 0.0,
        -n_cam_pos.x,    -n_cam_pos.y,   0.0, 1.0
    );

    mat4 scale_matrix = mat4(
        u_camera.scale,   0.0,            0.0, 0.0,
        0.0,            u_camera.scale,  0.0, 0.0,
        0.0,            0.0,            1.0, 0.0,
        0.0,            0.0,            0.0, 1.0
    );

    gl_Position = scale_matrix * transform_matrix * vec4(uv.xy, 0.0, 1.0);
}