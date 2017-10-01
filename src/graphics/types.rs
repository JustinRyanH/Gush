use gfx;
use gfx_device_gl as gfx_gl;

use graphics::pipeline;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;


pub type EncoderOGL = gfx::Encoder<gfx_gl::Resources, gfx_gl::CommandBuffer>;
pub type ColorViewOGL = gfx::handle::RenderTargetView<gfx_gl::Resources, ColorFormat>;
pub type DepthViewOGL = gfx::handle::DepthStencilView<gfx_gl::Resources, DepthFormat>;

pub type GpuProgram = gfx::handle::Program<gfx_gl::Resources>;
pub type GpuBuffer<T> = gfx::handle::Buffer<gfx_gl::Resources, T>;
pub type Texture<T> = gfx::handle::Texture<gfx_gl::Resources, T>;
pub type Sampler = gfx::handle::Sampler<gfx_gl::Resources>;
pub type Slice  = gfx::Slice<gfx_gl::Resources>;

pub type PipelineState<T> = gfx::pso::PipelineState<gfx_gl::Resources, T>;
pub type PipelineData = pipeline::gpu_pipeline::Data<gfx_gl::Resources>;
pub type PipelineMetadata = pipeline::gpu_pipeline::Meta;
