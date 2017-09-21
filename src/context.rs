
use glutin::{WindowBuilder, ContextBuilder, GlWindow, EventsLoop, GlProfile, Api, GlRequest};
use gfx_window_glutin as gfx_window;

use error::AppResult;
use gfx_context::{ColorFormat, DepthFormat};

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

pub struct Context {
    _window: GlWindow,
    event_buffer: EventsLoop,
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

        let (window, _, _, _, _) =
            gfx_window::init::<ColorFormat, DepthFormat>(window_builder, context, &event_buffer);

        Ok(Context {
            _window: window,
            event_buffer,
        })
    }
}

pub fn run(ctx: &mut Context) -> AppResult<()> {
    let ref mut event_buffer = ctx.event_buffer;

    let mut running = true;

    while running {
        use glutin::{ Event, WindowEvent };
        event_buffer.poll_events(|event| {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Closed => running = false,
                    _ => (),
                },
                _ => (),
            }
        })
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
