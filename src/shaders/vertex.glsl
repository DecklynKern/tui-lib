#version 140

in vec2 position;
out vec2 pos;

void main() {
    gl_Position = vec4(position.x, -position.y, 0.0, 1.0);
    pos = position;
}