#version 330 core

struct Light {
    vec2 position;
    float radius;
    float intensity;
};

out vec4 color;

in vec2 vertex_position;

uniform float u_radius;
uniform Light u_light;

void main(void) {
    float d = length(vertex_position.xy);

    if(d < u_radius - 10.0) {
        float light_cof = 0.0;
        vec2 dist = vertex_position - u_light.position;
        if (length(dist) < u_light.radius) {
            light_cof = smoothstep(0.005, 0.01, u_light.intensity/length(dist));
            light_cof /= 3.0;
        }

        color = vec4(0.16 + light_cof*0.9, 0.18 + light_cof, 0.19, 1.0);
    } else if(d < u_radius) {
        color = vec4(0.26, 0.28, 0.31, 1.0);
    } else {
        color = vec4(0.0);
    }

}