#version 330 core

layout (location = 0) in vec3 aPos;

uniform mat4 model = mat4(0.0);
uniform mat4 camera_pos = mat4(0.0);
uniform mat4 projection = mat4(0.0);

void main() {
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    if (model != mat4(0.0)) {
        gl_Position = model * gl_Position;
    }

    if (camera_pos != mat4(0.0)) {
        gl_Position = camera_pos * gl_Position;
    }
    if (projection != mat4(0.0)) {
        gl_Position = projection * gl_Position;
    }
}