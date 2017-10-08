use std::cell::RefCell;
use std::time::Instant;

use glutin::{self, WindowBuilder, ContextBuilder, GlWindow, EventsLoop, GlProfile, Api, GlRequest,
             GlContext};
use gfx_window_glutin as gfx_window;
use graphics::{data_pipeline, create_simple_pipeline};
use graphics::context::GfxContext;
use graphics::types::{ColorFormat, DepthFormat};
use graphics::mesh::SimpleMesh;
use error::AppResult;
use camera::Camera;
use vfs::VFS;

const CORNFLOWER_BLUE: [f32; 4] = [0.4, 0.58, 0.93, 1.];


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
            dimensions: [400, 300],
        }
    }
}

/// Handles Application Context including events, window, filesystem, and graphics
pub struct Context {
    pub window: GlWindow,
    pub event_buffer: EventsLoop,
    pub gfx: GfxContext,
    pub vfs: VFS,
    pub camera: Camera,
    pub epoch: Option<Instant>,
    pub last_instant: Option<Instant>,
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
        let epoch = Instant::now();

        Ok(Context {
            window,
            event_buffer,
            vfs,
            gfx,
            camera: Camera::new(),
            epoch: Some(epoch),
            last_instant: None,
        })
    }

    /// Tell the window to swap to the next rendering buffer.
    pub fn swap_buffer(&mut self) -> AppResult<()> {
        self.window.swap_buffers()?;
        Ok(())
    }

    pub fn resize(&mut self) {
        let (color_view, depth_view) = gfx_window::new_views(&self.window);
        self.gfx.color_view = color_view;
        self.gfx.depth_view = depth_view;
    }

    pub fn next_events(&mut self) -> Vec<glutin::Event> {
        let mut events = Vec::new();
        self.event_buffer.poll_events(|evt| events.push(evt));
        return events;
    }

    pub fn delta(&mut self) -> f64 {
        let last = match self.last_instant {
            Some(inst) => inst,
            None => Instant::now(),
        };
        let now = Instant::now();
        let delta = now - last;
        self.last_instant = Some(now);
        delta.as_secs() as f64 + (delta.subsec_nanos() as f64 * 1e-9)
    }

    pub fn since_program_epoch(&mut self) -> f64 {
        match self.epoch {
            Some(e) => {
                let since = e.elapsed();
                return since.as_secs() as f64 + (since.subsec_nanos() as f64 * 1e-9);
            }
            None => 0.,
        }
    }
}

pub fn run(ctx: &mut Context) -> AppResult<()> {
    let pipeline_state = create_simple_pipeline(ctx, "basic.vert", "basic.frag")?;

    let mesh = SimpleMesh::from_gltf(ctx, "Cube.gltf")?;
    let (vertex_buffer, slice) = mesh.generate_buffer(ctx)?;


    let mut data = data_pipeline(ctx, vertex_buffer, mesh.texture)?;
    let radius = 5.0;
    data.view = ctx.camera.as_matrix().into();

    let mut running = true;

    while running {
        ctx.swap_buffer()?;
        ctx.gfx.clear(CORNFLOWER_BLUE, &data);
        let cam_x = (ctx.since_program_epoch().sin() * radius) as f32;
        let cam_y = (ctx.since_program_epoch().cos() * radius) as f32;
        ctx.camera.move_to([cam_x, 0., cam_y].into());
        data.view = ctx.camera.as_matrix().into();
        ctx.gfx.draw(&pipeline_state, &data, &slice);
        ctx.gfx.cleanup();
        ctx.gfx.flush();

        use glutin::{VirtualKeyCode, Event, WindowEvent};
        let events = ctx.next_events();
        for event in events {
            match event {
                Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::Closed => running = false,
                        WindowEvent::Resized(_, _) => ctx.resize(),
                        WindowEvent::KeyboardInput { input, .. } => {
                            if let Some(key) = input.virtual_keycode {
                                match key {
                                    VirtualKeyCode::Escape => running = false,
                                    _ => (),
                                }
                            }
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }
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
