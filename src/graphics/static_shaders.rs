pub const FRAG_SHADER: &'static str = "
#version 330 core
out vec4 FragColor;
in vec2 TextureCoord;

uniform sampler2D u_texture;

void main()
{
  FragColor = texture(u_texture, TextureCoord); 
}
";

pub const VERT_SHADER: &'static str = "
#version 330 core

in vec3 a_pos;
in vec2 a_textureCoord;

out vec2 TextureCoord;

uniform mat4 u_model;
uniform mat4 u_view;
uniform mat4 u_projection;

void main()
{
  gl_Position = u_projection * u_view * u_model * vec4(a_pos, 1.0);
  TextureCoord = a_textureCoord;
}
";
