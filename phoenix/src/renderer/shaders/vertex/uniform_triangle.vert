#version 330 core

layout (location = 0) in vec3 aPos;

uniform mat4 translation = mat4(0.0);
uniform mat4 rotation = mat4(0.0);
uniform mat4 scale = mat4(0.0);

void main() {
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    if (rotation != mat4(0.0)) {
        gl_Position = rotation * gl_Position;
    }

    if (translation != mat4(0.0)) {
        gl_Position = translation * gl_Position;
    }
    if (scale != mat4(0.0)) {
        gl_Position = scale * gl_Position;
    }
}