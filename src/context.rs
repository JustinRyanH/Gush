use winit;

use error::AppResult;
use vfs::VFS;


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
    pub event_buffer: winit::EventsLoop,
    pub window: winit::Window,
    pub vfs: VFS,
}

impl Context {
    pub fn from_app_builder(builder: &AppConfig) -> AppResult<Context> {
        let event_buffer = winit::EventsLoop::new();

        let window = winit::WindowBuilder::new()
            .with_title(builder.title.to_owned())
            .with_dimensions(builder.dimensions[0], builder.dimensions[1])
            .build(&event_buffer)?;
        let vfs = VFS::new()?;
        Ok(Context {
            event_buffer,
            window,
            vfs,
        })
    }

    pub fn next_events(&mut self) -> Vec<winit::Event> {
        let mut events = Vec::new();
        self.event_buffer.poll_events(|evt| events.push(evt));
        return events;
    }
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
