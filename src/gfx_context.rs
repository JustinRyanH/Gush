use std::cell::RefCell;

use gfx;
use gfx::pso;
use gfx::handle::Buffer;
use gfx::traits::{ Factory, FactoryExt, Device };
use gfx_device_gl as gfx_gl;


use context::Context;
use error::{AppResult, AppError};


pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 3] = "aPos",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

pub struct GfxContext {
    /// Used to send commands into gpu CommandBuffer
    pub encoder: gfx::Encoder<gfx_gl::Resources, gfx_gl::CommandBuffer>,
    pub factory: RefCell<gfx_gl::Factory>,
    pub device: gfx_gl::Device,
    pub color_view: gfx::handle::RenderTargetView<gfx_gl::Resources, ColorFormat>,
    pub depth_view: gfx::handle::DepthStencilView<gfx_gl::Resources, DepthFormat>,
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

        GfxContext {
            factory,
            device,
            encoder,
            color_view,
            depth_view,
        }
    }

    /// Load shaders from shader directory and generate a shaderset to use
    pub fn load_and_compile_shaders(
        ctx: &mut Context,
        vertex: &str,
        fragment: &str,
    ) -> AppResult<gfx::handle::Program<gfx_gl::Resources>> {
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

    /// Clears the screen to the supplied color
    pub fn clear(&mut self, color: [f32; 4]) {
        self.encoder.clear(&self.color_view, color);
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
        AppResult<(Buffer<gfx_gl::Resources, Vertex>, gfx::Slice<gfx_gl::Resources>)>
        where I: gfx::IntoIndexBuffer<gfx_gl::Resources>
    {
        let mut factory = match self.factory.try_borrow_mut() {
            Ok(factory) => factory.clone(),
            Err(_) => {
                let err_location = format!("{}:{}", file!(), line!());
                return Err(AppError::MemError(
                    "Mutable Borrow Error".into(),
                    err_location,
                ));
            }
        };

        Ok(factory.create_vertex_buffer_with_slice(vertices, indices))
    }

    /// Generates the Graphics Pipeline to used to send data to the GPU
    pub fn data_pipeline(&mut self, buffer: Buffer<gfx_gl::Resources, Vertex>) -> pipe::Data<gfx_gl::Resources> {
        pipe::Data {
            vbuf: buffer,
            out: self.color_view.clone(),
        }
    }

    /// Draws tell GPU to draw object
    pub fn draw(&mut self, pipeline: &pso::PipelineState<gfx_gl::Resources, pipe::Meta>,
                data: &pipe::Data<gfx_gl::Resources>,
                indices: &gfx::Slice<gfx_gl::Resources>) {
        self.encoder.draw(indices, pipeline, data)
    }
}
