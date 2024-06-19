#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec4 in_color;
layout (location = 2) in vec2 in_texture_coord;

out vec2 text_coord;
out vec4 vertex_color;

void main()
{
    gl_Position = vec4(position, 1.0);

    text_coord = in_texture_coord;
    vertex_color = in_color;
}