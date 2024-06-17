#version 330 core

in vec2 text_coord;
out vec4 fragColor;
uniform sampler2D ourTexture;

void main()
{
    fragColor = texture(ourTexture, text_coord);
}
