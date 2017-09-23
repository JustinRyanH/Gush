use std::cell::RefCell;

use glutin::{WindowBuilder, ContextBuilder, GlWindow, EventsLoop, GlProfile, Api, GlRequest,
             GlContext};
use gfx::Primitive;
use gfx::traits::Factory;
use gfx::pso::{PipelineState, PipelineInit, Descriptor};
use gfx::state::Rasterizer;
use gfx_window_glutin as gfx_window;

use error::{ AppResult, AppError };
use vfs::VFS;
use gfx_context::{GfxContext, ColorFormat, DepthFormat, pipe, Vertex};

const CORNFLOWER_BLUE: [f32; 4] = [0.4, 0.58, 0.93, 1.];
const TRIANGLE: [Vertex; 3] = [
    Vertex { pos: [-0.9, -0.5, 0.] },
    Vertex { pos: [ 0.9, -0.5, 0.] },
    Vertex { pos: [-0.9,  0.5, 0.] },
];

/// Configuration for Application. This will eventually be able to loaded from a
/// toml configuration file
#[derive(Debug, PartialEq, Clone)]
pub struct AppConfig {
    title: String,
    dimensions: [u32; 2],
}

impl AppConfig {
    /// Create an app with default properties
    pub fn new() -> AppConfig {
        AppConfig::default()
    }

    /// Return new app with updated dimensions
    pub fn with_dimensions(self, width: u32, height: u32) -> AppConfig {
        AppConfig {
            dimensions: [width, height],
            ..self
        }
    }
    /// Return new app with updated title
    pub fn with_title(self, title: &'static str) -> AppConfig {
        AppConfig {
            title: title.to_owned(),
            ..self
        }
    }
}

impl Default for AppConfig {
    /// Create a default AppConfig Configuraiton
    fn default() -> AppConfig {
        AppConfig {
            title: "Default Gush AppConfig".to_owned(),
            dimensions: [400, 400],
        }
    }
}

/// Handles Application Context including events, window, filesystem, and graphics
pub struct Context {
    pub window: GlWindow,
    pub event_buffer: EventsLoop,
    pub gfx: GfxContext,
    pub vfs: VFS,
}

impl Context {
    pub fn from_app_builder(builder: &AppConfig) -> AppResult<Context> {
        let event_buffer = EventsLoop::new();
        let window_builder = WindowBuilder::new()
            .with_dimensions(builder.dimensions[0], builder.dimensions[1])
            .with_title(builder.title.to_owned());

        let context = ContextBuilder::new()
            .with_gl_profile(GlProfile::Core)
            .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
            .with_vsync(false);

        let (window, device, factory, color_view, depth_view) =
            gfx_window::init::<ColorFormat, DepthFormat>(window_builder, context, &event_buffer);

        let gfx = GfxContext::new(RefCell::new(factory), device, color_view, depth_view);

        let vfs = VFS::new()?;

        Ok(Context {
            window,
            event_buffer,
            vfs,
            gfx,
        })
    }

    /// Tell the window to swap to the next rendering buffer.
    pub fn swap_buffer(&mut self) -> AppResult<()> {
        self.window.swap_buffers()?;
        Ok(())
    }
}

pub fn run(ctx: &mut Context) -> AppResult<()> {
    let program = GfxContext::load_and_compile_shaders(ctx, "basic.vert", "basic.frag")?;

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


    let mut desc = Descriptor::new(Primitive::TriangleList, Rasterizer::new_fill());
    let meta = pipe::new().link_to(&mut desc, program.get_info())?;

    let pipeline_state = PipelineState::new(
        factory.create_pipeline_state_raw(&program, &desc)?,
        Primitive::TriangleList,
        meta.clone());

    let (vertex_buffer, slice) = ctx.gfx.generate_buffer(&TRIANGLE, ())?;
    let data = ctx.gfx.data_pipeline(vertex_buffer);

    let mut running = true;
    while running {
        ctx.swap_buffer()?;
        ctx.gfx.flush();
        ctx.gfx.clear(CORNFLOWER_BLUE);
        ctx.gfx.draw(&pipeline_state, &data, &slice);
        ctx.gfx.cleanup();

        use glutin::{Event, WindowEvent};
        &ctx.event_buffer.poll_events(|event| match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::Closed => running = false,
                    _ => (),
                }
            }
            _ => (),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn app_default() {
        assert_eq!(
            AppConfig {
                title: "Default Gush AppConfig".to_owned(),
                dimensions: [400, 400],
            },
            AppConfig::default()
        );
    }

    #[test]
    fn app_with_dimensions() {
        assert_eq!(
            AppConfig {
                dimensions: [600, 600],
                ..Default::default()
            },
            AppConfig::new().with_dimensions(600, 600)
        )
    }

    #[test]
    fn app_with_title() {
        assert_eq!(
            AppConfig {
                title: "A Different Title".to_owned(),
                ..Default::default()
            },
            AppConfig::new().with_title("A Different Title")
        )
    }
}
