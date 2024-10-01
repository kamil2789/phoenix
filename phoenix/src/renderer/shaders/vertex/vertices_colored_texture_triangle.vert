#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec4 in_color;
layout (location = 2) in vec2 in_texture_coord;

uniform mat4 model = mat4(0.0);
uniform mat4 camera_pos = mat4(0.0);
uniform mat4 projection = mat4(0.0);

out vec2 text_coord;
out vec4 vertex_color;

void main()
{
    gl_Position = vec4(position, 1.0);
    if (model != mat4(0.0)) {
        gl_Position = model * gl_Position;
    }

    if (camera_pos != mat4(0.0)) {
        gl_Position = camera_pos * gl_Position;
    }

    if (projection != mat4(0.0)) {
        gl_Position = projection * gl_Position;
    }

    text_coord = in_texture_coord;
    vertex_color = in_color;
}