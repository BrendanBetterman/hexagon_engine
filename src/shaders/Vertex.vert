#version 140
    
uniform mat4 persp_matrix;
uniform mat4 view_matrix;
uniform mat4 rot_x_matrix;
uniform mat4 rot_y_matrix;
in vec3 position;
in vec3 normal;
in vec2 texture;

out vec3 v_position;
out vec3 v_normal;
out vec2 v_tex_coords;

void main() {
    v_tex_coords = texture;
    v_position = position;
    v_normal = normal;
    gl_Position = persp_matrix * (rot_x_matrix*(rot_y_matrix * view_matrix * vec4(v_position * 0.005, 1.0)));
}