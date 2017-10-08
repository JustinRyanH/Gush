pub mod types;
pub mod context;
pub mod pipeline;
pub mod mesh;

use gfx;
use gfx::texture::{ self, SamplerInfo };
use gfx::traits::{Factory, FactoryExt};
use cgmath::{self, Deg, Transform};

use context::Context;
use error::{ AppResult, AppError };
use graphics::types::{PipelineMetadata, PipelineState};
use graphics::pipeline::{gpu_pipeline, Vertex};
use texture::Texture;

/// Loads Shaders from VFS and Creates a shader program for the GPU
pub fn load_and_compile_shaders(
    ctx: &mut Context,
    vertex: &str,
    fragment: &str,
) -> AppResult<types::GpuProgram> {
    let mut factory = ctx.gfx.get_factory_clone()?;
    let vertex_bin = ctx.vfs.load_shader_code(vertex)?;
    let fragment_bin = ctx.vfs.load_shader_code(fragment)?;
    let vertex_shader = factory.create_shader_vertex(vertex_bin.as_bytes())?;
    let fragment_shader = factory.create_shader_pixel(fragment_bin.as_bytes())?;

    match factory.create_program(&gfx::ShaderSet::Simple(vertex_shader, fragment_shader)) {
        Ok(p) => Ok(p),
        Err(e) => Err(e.into()),
    }
}

pub fn create_simple_pipeline(ctx: &mut Context, vertex: &str, fragment: &str) -> AppResult<PipelineState<PipelineMetadata>> {
    let mut factory = ctx.gfx.get_factory_clone()?;
    match factory.create_pipeline_simple(
        ctx.vfs.load_shader_code(vertex)?.as_bytes(),
        ctx.vfs.load_shader_code(fragment)?.as_bytes(),
        gpu_pipeline::new()
    ) {
        Ok(k) => Ok(k),
        Err(e) => Err(e.into()),
    }
}

/// Generates the Graphics Pipeline to used to send data to the GPU
pub fn data_pipeline(
    ctx: &mut Context,
    buffer: types::GpuBuffer<Vertex>,
    texture: Option<Texture>,
) -> AppResult<types::PipelineData> {
    let sampler = ctx.gfx.get_factory_clone()?.create_sampler(SamplerInfo::new(
        texture::FilterMethod::Trilinear,
        texture::WrapMode::Tile,
    ));
    let tex: Texture = match texture {
        Some(t) => t,
        None => Texture::from_memory(ctx, 2, 2, &[0; 4])?,
    };
    let aspect_ratio =  match ctx.window.get_inner_size_pixels() {
        Some(( w, h )) => w as f32 /  h as f32,
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

