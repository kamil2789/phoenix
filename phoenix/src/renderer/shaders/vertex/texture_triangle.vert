#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec2 in_texture_coord;

out vec2 text_coord;

void main()
{
    gl_Position = vec4(position, 1.0);
    text_coord = in_texture_coord;
}