extern crate gush;

use gush::context;

fn main() {
    println!("{}", "Breakout Example");

    let mut ctx = context::Context::from_app_builder(&context::AppConfig::default()).unwrap();
    if let Err(e) = context::run(&mut ctx) {
        println!("Error: {}", e);
    } else {
        println!("Game exited cleanly");
    }

}
