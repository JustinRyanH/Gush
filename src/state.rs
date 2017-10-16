use glium::Surface;

use context::Context;
use error::AppResult;

pub enum Next {
    /// No Major world altering actions taken
    None,
    /// Quits the Current State
    Quit,
}

pub trait StateEngine {
    fn start(&mut self, _ctx: &mut Context) -> AppResult<()> {
        Ok(())
    }
    fn stop(&mut self, _ctx: &mut Context) -> AppResult<()> {
        Ok(())
    }
    fn update(&mut self, _ctx: &mut Context) -> AppResult<Next> {
        Ok(Next::None)
    }
    fn draw(&mut self, _ctx: &mut Context) -> AppResult<()> {
        Ok(())
    }
}

pub fn run(ctx: &mut Context, engine: &mut StateEngine) -> AppResult<()> {
    engine.start(ctx)?;
    let mut running = true;
    while running {
        let mut target = ctx.window.draw();
        target.clear_color(0., 0., 1., 1.);
        target.finish()?;
        engine.draw(ctx)?;

        use winit::{Event, WindowEvent};
        let events = ctx.next_events();
        for event in events {
            match event {
                Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::Closed => running = false,
                        _ => (),
                    }
                },
                _ => (),
            }
        }
        engine.update(ctx)?;
    }
    engine.stop(ctx)?;
    Ok(())
}
