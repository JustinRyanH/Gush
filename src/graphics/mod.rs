pub mod types;
pub mod context;
pub mod pipeline;
pub mod mesh;
pub mod static_shaders;

pub use graphics::mesh::SimpleMesh as Mesh;

use gfx::texture::{self, SamplerInfo};
use gfx::traits::{Factory};
use cgmath::{self, Deg, Transform};

use context::Context;
use error::{AppResult, AppError};
use graphics::pipeline::{gpu_pipeline, Vertex};
use texture::Texture;


/// Generates the Graphics Pipeline to used to send data to the GPU
pub fn data_pipeline(
    ctx: &mut Context,
    buffer: types::GpuBuffer<Vertex>,
    texture: Option<Texture>,
) -> AppResult<types::PipelineData> {
    let sampler = ctx.gfx.get_factory_clone()?.create_sampler(
        SamplerInfo::new(
            texture::FilterMethod::Trilinear,
            texture::WrapMode::Tile,
        ),
    );
    let tex: Texture = match texture {
        Some(t) => t,
        None => Texture::from_memory(ctx, 2, 2, &[0; 4])?,
    };
    let aspect_ratio = match ctx.window.get_inner_size_pixels() {
        Some((w, h)) => w as f32 / h as f32,
        None => return Err(AppError::GfxError("Window no longer exists".into())),
    };

    Ok(gpu_pipeline::Data {
        vbuf: buffer,
        out: ctx.gfx.color_view.clone(),
        out_depth: ctx.gfx.depth_view.clone(),
        texture: (tex.resource_view, sampler),
        view: cgmath::Matrix4::one().into(),
        projection: cgmath::perspective::<f32, Deg<f32>>(Deg(45.0), aspect_ratio, 0.1, 100.).into(),
        model: cgmath::Matrix4::one().into(),
    })
}

