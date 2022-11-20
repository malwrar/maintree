#version 330 core

layout (location = 0) in vec3 vert_pos;
layout (location = 1) in vec2 vert_tx;

out VERTEX_OUT {
    vec2 tx_pos;
} OUT;

void main() {
    gl_Position = vec4(vert_pos, 1.0);
    OUT.tx_pos = vert_tx;
}