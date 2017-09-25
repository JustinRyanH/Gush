#version 330 core
out vec4 FragColor;
in vec2 TextureCoord;
in vec4 vertexColor;

uniform sampler2D aTexture;

void main()
{
  FragColor = texture(aTexture, TextureCoord) * vertexColor;
}
