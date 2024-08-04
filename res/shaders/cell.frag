#version 330 core

precision lowp float;

in vec2 st;

// uniform vec2 u_resolution;
uniform float u_time;

vec2 random2( vec2 p ) {
    return fract(sin(vec2(dot(p,vec2(127.1,311.7)),dot(p,vec2(269.5,183.3))))*43758.5453);
}

void render_cell(out vec4 color, float d, vec2 st, float m_dist, float radius) {
    bool is_cell_body = d < radius;

    if (is_cell_body) {
        // bg of cell
        color = vec4(0.972,0.972,0.972, 1.0);
        
        // смежная оболочка
        if (smoothstep(0.136, 0.232, m_dist*(sin(d))) <= 0.520)
        	color = vec4(0.675,0.675,0.675,1.0);
        
        // ЭПС
    	if (d < radius - 4.0 
            && smoothstep(0.12, 0.9, sin(m_dist*cos(-d*d/3.552))) <= 0.256)
        	color = vec4(0.799,0.799,0.799, 1.0);
        
        // жидкость ядра
    	if (d < radius - 2.0 
            && smoothstep(0.136, 0.232, m_dist*(d*d*d*0.024)) <= 0.960)
    		color = vec4(0.425,0.425,0.425, 1.0);
        
        // круг ядра
    	if (d < 2.544)
    		color = vec4(0.441,0.441,0.441, 1.0) * 
            smoothstep(-1.144, 1.596, 1.768-d/2.3);
        
        // оболочка вокруг ядра
    	if (smoothstep(0.136, 1.232, sin(m_dist)) <= 0.088)
    		color = vec4(0.675,0.675,0.675, 1.0);
        
        color -= vec4((1.352 - smoothstep(1.328, 0.668, d/10.0))/1.936);
    }	
}

void main() {
    vec2 st = st;
    st *= 20.0;
    
    vec4 color = vec4(0.0);
    
    vec2 pos_center = vec2(10.0);
    vec2 points[1];
    points[0] = pos_center;
    

    // Tile the space
    vec2 i_st = floor(st);
    vec2 f_st = fract(st);

    float radius = 10.0;
    float m_dist = 2.356;  // minimum distance
    for (int j= -1; j <= 1; j++ ) {
        for (int i= -1; i <= 1; i++ ) {
            // Neighbor place in the grid
            vec2 neighbor = vec2(float(i),float(j));
            // Random position from current + neighbor place in the grid
            vec2 offset = random2(i_st + neighbor);
            // Animate the offset
            offset = 0.5 + 0.5*sin(u_time + 6.28*offset);
            // Position of the cell
            vec2 pos = neighbor + offset - f_st;
            // Cell distance
            float dist = length(pos);
            // Metaball
            m_dist = min(m_dist, m_dist*dist);
        }
    }

    float m_dist2 = 10.0;  // minimum distance

    for (int i = 0; i < 1; i++) {
        float dist = distance(st, points[i]);
        m_dist2 = min(m_dist2, dist);
    }
    
    // Draw cell
    float d1 = distance(pos_center, st);
    if (d1 - m_dist2 < .1)
    render_cell(color, d1, st, m_dist, radius);
    
    gl_FragColor = color;
}
