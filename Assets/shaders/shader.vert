#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;
out vec2 texCoord;
uniform vec3 offset;
void main()
{

    vec3 position = aPos + offset;
    position.x = position.x / 1920.0 * 2 - 1;
    position.y = 1 - (position.y/1080.0) * 2;
    gl_Position = vec4(position, 1.0);
    texCoord = aTexCoord;
}