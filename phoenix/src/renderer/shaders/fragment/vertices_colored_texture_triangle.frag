#version 330 core

in vec2 text_coord;
in vec4 vertex_color;

out vec4 fragColor;

uniform sampler2D ourTexture;

void main()
{
    fragColor = texture(ourTexture, text_coord) * vertex_color;

}