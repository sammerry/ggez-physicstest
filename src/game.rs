use ggez;
use ggez::event::{Events, EventHandler, MouseButton, Keycode, Mod};
use ggez_goodies::scene::{SceneStack, Scene};
use ggez_goodies::input;

use nalgebra::Vector2;
use ncollide2d::world::CollisionGroups;

use crate::world::SceneWorld;
use crate::scene::menu::MenuScene;
use crate::scene::physicstest::PhysicsTest;

use std::boxed::Box;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Button {
    Shoot,
    Rotate,
    Gravity,
    Quit,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Axis {
    Forward,
    Backward,
}


pub type InputBinding = input::InputBinding<Axis, Button>;
pub type InputEvent = input::InputEffect<Axis, Button>;
pub type InputState = input::InputState<Axis, Button>;

pub fn bind_inputs() -> input::InputBinding<Axis, Button> {
    input::InputBinding::new()
        .bind_key_to_axis(Keycode::Up, Axis::Forward, true)
        .bind_key_to_axis(Keycode::Down, Axis::Backward, false)
        .bind_key_to_button(Keycode::G, Button::Gravity)
        .bind_key_to_button(Keycode::Z, Button::Rotate)
        .bind_key_to_button(Keycode::Space, Button::Shoot)
        .bind_key_to_button(Keycode::Escape, Button::Quit)
}

/**
 * The global game state. There should only be one. This tracks total scenes
 * and the input bindings.
 */
pub struct Game {
    sceneStack: SceneStack<SceneWorld, InputEvent>,
    input_binding: InputBinding,
}

impl Game {
    pub fn new(context: &mut ggez::Context) -> Game {
        debug!("Creating Game State");

        // set background color
        ggez::graphics::set_background_color(context, ggez::graphics::BLACK);

        // Create the scenes in the game
        let mut sceneWorld = SceneWorld::new(context);
        let mut sceneStack = SceneStack::new(context, sceneWorld);
        sceneStack.push(Box::new(PhysicsTest::new()));
        sceneStack.push(Box::new(MenuScene::new()));

        Game {
            sceneStack: sceneStack,
            input_binding: bind_inputs(),
        }
    }
}

///
/// Event Handler
/// triggered on all events. We use the InputBindings to take action
///
impl EventHandler for Game {

    fn draw(self: &mut Self, context: &mut ggez::Context) -> ggez::GameResult<()> {
        ggez::graphics::clear(context);
        self.sceneStack.draw(context);
        ggez::graphics::present(context);
        Ok(())
    }

    fn update(self: &mut Self, context: &mut ggez::Context) -> ggez::GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while ggez::timer::check_update_time(context, DESIRED_FPS) {
            self.sceneStack.update();
        }

        if self.sceneStack.world.quit {
            info!("Exiting due to world quit flag.");
            context.quit();
        }

        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut ggez::Context, button: MouseButton, x: i32, y: i32) {
        if let Some(ev) = self.input_binding.resolve(keycode) {
            self.sceneStack.world.input.update_effect(ev, true);
            self.sceneStack.input(ev, true);
        }
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut ggez::Context, button: MouseButton, x: i32, y: i32) {
        println!("Mouse button released: {:?}, x: {}, y: {}", button, x, y);
    }

    fn key_down_event(
        self: &mut Self,
        context: &mut ggez::Context,
        keycode: Keycode,
        _keymod: Mod,
        _repeat: bool
    ) {
        if let Some(ev) = self.input_binding.resolve(keycode) {
            self.sceneStack.world.input.update_effect(ev, true);
            self.sceneStack.input(ev, true);
        }
    }

    fn key_up_event(
        self: &mut Self,
        context: &mut ggez::Context,
        keycode: Keycode,
        _keymod: Mod,
        _repeat: bool
    ) {
        if let Some(ev) = self.input_binding.resolve(keycode) {
            self.sceneStack.world.input.update_effect(ev, false);
            self.sceneStack.input(ev, false);
        }
    }
}

