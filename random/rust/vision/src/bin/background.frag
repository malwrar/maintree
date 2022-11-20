#version 330 core

in VERTEX_OUT {
    vec2 tx_pos;
} IN;

uniform sampler2D window_content;

out vec4 frag_color;

void main() {
    frag_color = vec4(texture(window_content, IN.tx_pos).xyz, 1.0);
}