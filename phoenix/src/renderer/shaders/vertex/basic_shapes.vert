#version 330 core

layout (location = 0) in vec3 position;
layout (location = 2) in vec4 in_color;
layout (location = 3) in vec2 in_texture_coord;

uniform mat4 model = mat4(1.0);
uniform mat4 camera_pos = mat4(1.0);
uniform mat4 projection = mat4(1.0);

uniform int is_texture_vert = 0;
uniform int is_color_vert = 0;

out vec2 text_coord;
out vec4 vertex_color;

void main()
{
    gl_Position = projection * camera_pos * model * vec4(position, 1.0);

    if (is_texture_vert == 1) {
        text_coord = in_texture_coord;
    }

    if (is_color_vert == 1) {
        vertex_color = in_color;
    }
}