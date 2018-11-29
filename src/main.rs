
extern crate chrono;
extern crate fern;
#[macro_use]
extern crate log;

extern crate specs;
#[macro_use]
extern crate specs_derive;
#[macro_use]
extern crate derive_deref;

extern crate ggez;
extern crate ggez_goodies;

extern crate nalgebra;
extern crate nphysics2d;
extern crate ncollide2d;

use ggez::ContextBuilder;
use ggez::conf::{WindowSetup, WindowMode};

mod scene;
mod world;
mod game;
mod system;

use self::game::Game;

/// Default rust logging with fern. Logs to stdout and can handle colors if needed
fn enable_logging() {
    fern::Dispatch::new()
    // Perform allocation-free log formatting
    .format(|out, message, record| {
        out.finish(format_args!(
            "{}[{}][{}] {}",
            chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
            record.target(),
            record.level(),
            message
        ))
    })

    // Add blanket level filter -
    .level(log::LevelFilter::Debug)
    // Output to stdout, files, and other Dispatch configurations
    .chain(std::io::stdout())
    // Apply globally
    .apply();
}


fn main() {
    enable_logging();

    info!("Starting Game");

    // Define the window
    let mut cb = ContextBuilder::new("testing game", "asdf")
        .window_setup(WindowSetup::default().title("BLAH"))
        .window_mode(WindowMode::default().dimensions(800, 600));

    // Build the window context
    let context = &mut cb.build().unwrap();

    // build the game context
    let mut game = Game::new(context);

    // Run the game until quit or no scenes left
    if let Err(e) = ggez::event::run(context, &mut game) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}

