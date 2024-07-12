#version 140

in vec3 v_normal;
in vec2 v_tex_coords;
in vec3 v_position;

out vec4 f_color;

uniform sampler2D grass_tex;
uniform sampler2D rock_tex;
uniform sampler2D sand_tex;

 float min_rock_slope = 0.5;
 float max_grass_slope = 0.9;

 float min_rockgrass_height = -10f;
 float max_sand_height = 0.25f;

const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);


void main() {
    vec3 grass_albedo = texture(grass_tex, v_tex_coords).rgb;
    vec3 rock_albedo = texture(rock_tex, v_tex_coords).rgb;
    vec3 sand_albedo = texture(sand_tex, v_tex_coords).rgb;

    float rock_grass_weight = v_normal.y;
    float sand_rockgrass_weight = v_position.y;
    
    rock_grass_weight = max(min_rock_slope, rock_grass_weight);
    rock_grass_weight = min(max_grass_slope, rock_grass_weight);
    rock_grass_weight -= min_rock_slope;
    rock_grass_weight /= max_grass_slope - min_rock_slope;

    vec3 rockgrass_albedo = mix(rock_albedo,grass_albedo,rock_grass_weight);

    sand_rockgrass_weight = max(min_rockgrass_height, sand_rockgrass_weight);
    sand_rockgrass_weight = min(max_sand_height, sand_rockgrass_weight);
    sand_rockgrass_weight -= min_rockgrass_height;
    sand_rockgrass_weight /= max_sand_height - min_rockgrass_height;
    
    vec3 albedo = mix(sand_albedo,rockgrass_albedo,sand_rockgrass_weight); 

    float lum = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);
    vec3 color = (0.3 + 0.7 * lum) * albedo;
    gl_FragColor = vec4(color, 1.0);
}