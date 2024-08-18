#version 330 core
struct Camera {
    vec2 position;
    float scale;
};

in vec2 vert_pos;
uniform vec2 u_resolution;
uniform Camera u_camera;


void main(void) {
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