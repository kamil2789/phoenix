#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec4 in_color;
layout (location = 2) in vec2 in_texture_coord;

uniform mat4 translation = mat4(0.0);
uniform mat4 rotation = mat4(0.0);
uniform mat4 scale = mat4(0.0);
uniform mat4 projection = mat4(0.0);

out vec2 text_coord;
out vec4 vertex_color;

void main()
{
    gl_Position = vec4(position, 1.0);
    if (rotation != mat4(0.0)) {
        gl_Position = rotation * gl_Position;
    }

    if (translation != mat4(0.0)) {
        gl_Position = translation * gl_Position;
    }

    if (scale != mat4(0.0)) {
        gl_Position = scale * gl_Position;
    }

    if (projection != mat4(0.0)) {
        gl_Position = projection * gl_Position;
    }

    text_coord = in_texture_coord;
    vertex_color = in_color;
}