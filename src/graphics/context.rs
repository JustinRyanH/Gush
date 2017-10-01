use std::cell::RefCell;

use gfx;
use gfx::state::Rasterizer;
use gfx::pso::Descriptor;
use gfx::traits::{ Factory, FactoryExt, Device };
use gfx_device_gl as gfx_gl;
use graphics::pipeline::{Vertex};
use graphics::types::{self, DepthFormat, ColorFormat};


use context::Context;
use error::{AppResult, AppError};

pub struct GfxContext {
    /// Used to send commands into gpu CommandBuffer
    pub encoder: types::EncoderOGL,
    pub factory: RefCell<gfx_gl::Factory>,
    pub device: gfx_gl::Device,
    pub color_view: types::ColorViewOGL,
    pub depth_view: types::DepthViewOGL,
    pub default_descriptor: Descriptor,
}

impl GfxContext {
    /// Create new GfxContext and generate an encoder for it
    pub fn new(
        factory: RefCell<gfx_gl::Factory>,
        device: gfx_gl::Device,
        color_view: gfx::handle::RenderTargetView<gfx_gl::Resources, ColorFormat>,
        depth_view: gfx::handle::DepthStencilView<gfx_gl::Resources, DepthFormat>,
    ) -> GfxContext {
        let encoder = factory.borrow_mut().create_command_buffer().into();
        let default_descriptor = Descriptor::new(gfx::Primitive::TriangleList, Rasterizer::new_fill());

        GfxContext {
            factory,
            device,
            encoder,
            color_view,
            depth_view,
            default_descriptor,
        }
    }

    /// Load shaders from shader directory and generate a shaderset to use
    pub fn load_and_compile_shaders(
        ctx: &mut Context,
        vertex: &str,
        fragment: &str,
    ) -> AppResult<types::GpuProgram> {
        let mut factory = match ctx.gfx.factory.try_borrow_mut() {
            Ok(factory) => factory.clone(),
            Err(_) => {
                let err_location = format!("{}:{}", file!(), line!());
                return Err(AppError::MemError(
                    "Mutable Borrow Error".into(),
                    err_location,
                ));
            }
        };
        let vertex_bin = ctx.vfs.load_shader_code(vertex)?;
        let fragment_bin = ctx.vfs.load_shader_code(fragment)?;
        let vertex_shader = factory.create_shader_vertex(vertex_bin.as_bytes())?;
        let fragment_shader = factory.create_shader_pixel(fragment_bin.as_bytes())?;

        match factory.create_program(&gfx::ShaderSet::Simple(vertex_shader, fragment_shader)) {
            Ok(p) => Ok(p),
            Err(e) => Err(e.into()),
        }
    }

    pub fn get_factory_clone(&mut self) -> AppResult<gfx_gl::Factory> {
        match self.factory.try_borrow_mut() {
            Ok(factory) => Ok(factory.clone()),
            Err(_) => {
                let err_location = format!("{}:{}", file!(), line!());
                return Err(AppError::MemError(
                    "Mutable Borrow Error".into(),
                    err_location,
                ));
            }
        }
    }

    /// Clears the screen to the supplied color
    pub fn clear(&mut self, color: [f32; 4] , data: &types::PipelineData) {
        self.encoder.clear(&self.color_view, color);
        self.encoder.clear_depth(&data.out_depth, 1.);
    }

    /// Cleans ununsed resources from the GPU
    pub fn cleanup(&mut self) {
        self.device.cleanup();
    }

    /// Sends the queued commands to the GPU. This should be done once per frame.
    pub fn flush(&mut self) {
        self.encoder.flush(&mut self.device);
    }

    /// Loads vertices into a GFX buffer for the GPU
    pub fn generate_buffer<I>(&mut self, vertices: &[Vertex], indices: I) ->
        AppResult<(types::GpuBuffer<Vertex>, types::Slice)>
        where I: gfx::IntoIndexBuffer<gfx_gl::Resources>
    {
        let mut factory = self.get_factory_clone()?;
        Ok(factory.create_vertex_buffer_with_slice(vertices, indices))
    }


    /// Draws tell GPU to draw object
    pub fn draw(&mut self, pipeline: &types::PipelineState<types::PipelineMetadata>,
                data: &types::PipelineData,
                indices: &types::Slice) {
        self.encoder.draw(indices, pipeline, data)
    }
}
