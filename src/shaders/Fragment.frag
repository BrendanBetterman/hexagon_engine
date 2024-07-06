#version 140
    
in vec3 v_normal;
in vec2 v_tex_coords;

out vec4 f_color;

uniform sampler2D diffuse_tex;

const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);


void main() {
    vec3 diffuse_color = texture(diffuse_tex, v_tex_coords).rgb;

    float lum = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);
    vec3 color = (0.3 + 0.7 * lum) * diffuse_color;//vec3(0.988,0.906,0.384);
    //gl_FragColor = vec4(1.0, 0.0, 0.9, 1.0) * (0.3+0.7*lum);
    gl_FragColor = vec4(color, 1.0);
}