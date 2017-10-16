extern crate gush;


use gush::context::{self, Context};
use gush::error::AppResult;
use gush::state::{ self, StateEngine };

const CORNFLOWER_BLUE: [f32; 4] = [0.4, 0.58, 0.93, 1.];

struct Breakout {
}

impl StateEngine for Breakout {
    fn start(&mut self, _ctx: &mut Context) -> AppResult<()> {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> AppResult<()> {
        Ok(())
    }
}

fn main() {
    println!("{}", "Breakout Example");

    let mut breakout = Breakout{};
    let mut ctx = context::Context::from_app_builder(&context::AppConfig::default()).unwrap();
    if let Err(e) = state::run(&mut ctx, &mut breakout) {
        println!("Error: {}", e);
    } else {
        println!("Game exited cleanly");
    }

}
