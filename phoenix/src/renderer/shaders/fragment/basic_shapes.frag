#version 330 core

in vec2 text_coord;
in vec4 vertex_color;
in vec3 normal;
in vec3 frag_pos;

out vec4 frag_color;

uniform sampler2D ourTexture;
uniform int is_texture_vert = 0;
uniform int is_color_vert = 0;
uniform vec4 color = vec4(1.0);

//light
uniform vec3 light_color = vec3(1.0);
uniform vec3 light_pos = vec3(0.0);
//view == camera
uniform vec3 view_pos = vec3(0.0);
uniform int is_light = 0;

vec3 calculate_light();

void main()
{
    vec4 our_color = color;
    if (is_color_vert == 1) {
        our_color = vertex_color;
    }

    if (is_texture_vert == 1) {
        frag_color = texture(ourTexture, text_coord) * our_color;
    } else {
        frag_color = our_color;
    }

    if (is_light == 1) {
        vec3 light_result = calculate_light();
        frag_color = vec4(light_result, 1.0) * frag_color;
    }
}

vec3 calculate_light() {
    float ambient_strength = 0.05;
    vec3 ambient = ambient_strength * light_color;

    // diffuse
    vec3 norm = normalize(normal);
    vec3 light_dir = normalize(light_pos - frag_pos);
    float diff = max(dot(norm, light_dir), 0.0);
    vec3 diffuse = diff * light_color;

    // specular
    float specular_strength = 0.5;
    vec3 view_dir = normalize(view_pos - frag_pos);
    vec3 reflect_dir = reflect(-light_dir, norm);
    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), 32);
    vec3 specular = specular_strength * spec * light_color;

    return ambient + diffuse + specular;
}