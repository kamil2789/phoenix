#version 330 core

in vec2 text_coord;
in vec4 vertex_color;

out vec4 fragColor;

uniform sampler2D ourTexture;
uniform int is_texture_vert;
uniform int is_color_vert = 0;
uniform vec4 color = vec4(1.0);

void main()
{
    vec4 our_color = color;
    if (is_color_vert == 1) {
        our_color = vertex_color;
    }

    if (is_texture_vert == 1) {
        fragColor = texture(ourTexture, text_coord) * our_color;
    } else {
        fragColor = our_color;
    }
}