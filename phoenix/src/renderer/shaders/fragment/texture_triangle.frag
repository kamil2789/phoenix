#version 330 core

in vec2 text_coord;
out vec4 fragColor;
uniform sampler2D ourTexture;

uniform vec4 color;
uniform int isUniformColor = -1;

void main()
{
    if (isUniformColor == 1) {
        fragColor = texture(ourTexture, text_coord) * color;
    } else {
        fragColor = texture(ourTexture, text_coord);
    }
}
