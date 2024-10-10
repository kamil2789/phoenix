#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec4 in_color;

uniform mat4 model = mat4(1.0);
uniform mat4 camera_pos = mat4(1.0);
uniform mat4 projection = mat4(1.0);

out vec4 color;

void main()
{
    gl_Position = projection * camera_pos * model * vec4(position, 1.0);
    color = in_color;
}