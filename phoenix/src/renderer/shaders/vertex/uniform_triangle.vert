#version 330 core

layout (location = 0) in vec3 aPos;

uniform mat4 model = mat4(1.0);
uniform mat4 camera_pos = mat4(1.0);
uniform mat4 projection = mat4(1.0);

void main() {
    gl_Position = projection * camera_pos * model * vec4(aPos, 1.0);
}