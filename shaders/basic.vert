#version 330 core

in vec3 a_pos;
in vec4 a_color;
in vec2 a_textureCoord;

out vec2 TextureCoord;
out vec4 vertexColor;

void main()
{
    gl_Position = vec4(a_pos.x, a_pos.y, a_pos.z, 1.0);
    TextureCoord = a_textureCoord;
    vertexColor = a_color;
}
