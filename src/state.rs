use context::Context;
use error::AppResult;

pub enum Next {
    /// No Major world altering actions taken
    None,
    /// Quits the Current State
    Quit,
}

pub trait StateEngine {
    fn start(&mut self, _ctx: &mut Context) -> AppResult<()> { Ok(()) }
    fn stop(&mut self, _ctx: &mut Context) -> AppResult<()> { Ok(()) }
    fn update(&mut self, _ctx: &mut Context) -> AppResult<Next> { Ok(Next::None) }
    fn draw(&mut self, _ctx: &mut Context) -> AppResult<()> { Ok(()) }
}
