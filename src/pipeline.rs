use gfx;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 3] = "a_pos",
        color: [f32; 4] = "a_color",
        uv:  [f32; 2] = "a_textureCoord",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "FragColor",
        texture: gfx::TextureSampler<[f32; 4]> = "aTexture",
    }
}
