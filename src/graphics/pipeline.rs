use gfx;

use graphics::types::{ColorFormat, DepthFormat};



gfx_defines!{
    vertex Vertex {
        pos: [f32; 3] = "a_pos",
        uv:  [f32; 2] = "a_textureCoord",
    }

    pipeline gpu_pipeline {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "FragColor",
        out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
        texture: gfx::TextureSampler<[f32; 4]> = "u_texture",
        model: gfx::Global<[[f32; 4]; 4]> = "u_model",
        view: gfx::Global<[[f32; 4]; 4]> = "u_view",
        projection: gfx::Global<[[f32; 4]; 4]> = "u_projection",
    }
}

impl Vertex {
    pub fn new(pos: [f32; 3], uv: [f32; 2]) -> Vertex {
        Vertex { pos, uv }
    }
}
