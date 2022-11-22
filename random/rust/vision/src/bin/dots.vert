#version 330 core

layout (location = 0) in vec3 vert_pos;

uniform mat4 modelview;
uniform mat4 projection;

//out VERTEX_OUT {
//} OUT;

void main() {
    gl_Position = projection * modelview * vec4(vert_pos, 1.0);
}