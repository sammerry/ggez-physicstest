use ggez::graphics;
use ggez_goodies::scene::{Scene, SceneSwitch};

use crate::game::{Button, InputEvent};
use crate::world::SceneWorld;

use log::*;

pub struct MenuScene {
    done: bool
}

impl MenuScene {
    pub fn new () -> MenuScene {
        debug!("Create MenuScene");

        MenuScene {
            done: false
        }
    }
}

impl Scene<SceneWorld, InputEvent> for MenuScene {
    fn update(&mut self, world: &mut SceneWorld) -> SceneSwitch<SceneWorld, InputEvent> {
        if self.done {
            // See https://github.com/ggez/ggez-goodies/issues/11
            SceneSwitch::Pop
        } else {
            SceneSwitch::None
        }
    }

    fn draw(&mut self, _gameworld: &mut SceneWorld, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        let t = ggez::graphics::TextCached::new(r#"
This is a collision test to verify ggez and nphysics2d.

- use spacebar to apply force to all balls
- use z to add more balls
- use g to toggle gravity (default off)

Press SPACEBAR to continue.
"#)?;

        t.queue(ctx, graphics::Point2::new(200.0, 100.0), Some(graphics::WHITE));

        graphics::TextCached::draw_queued(ctx, graphics::DrawParam::default())?;
        Ok(())
    }

    fn name(&self) -> &str {
        "MenuScene"
    }

    fn input(&mut self, world: &mut SceneWorld, _ev: InputEvent, _started: bool) {
        if world.input.get_button_pressed(Button::Shoot) {
            self.done = true;
        }
    }
}

