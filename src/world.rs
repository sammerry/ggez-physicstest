use nalgebra::Vector2;
use ncollide2d::world::CollisionGroups;

use specs::World as SpecsWorld;

use crate::game::InputState;

use ggez_goodies::input::InputState as GInputState;

pub struct SceneWorld {
    pub specs: specs::World,
    pub input: InputState,
    pub quit: bool,
}

/**
 * The SceneWorld, A snapshot of the world as the scene knows it.
 */
pub type PhysicsWorld = nphysics2d::world::World<f32>;
impl SceneWorld {
    pub fn new (context: &mut ggez::Context) -> SceneWorld {
        debug!("Creating Game State World");

        let mut physicsWorld: PhysicsWorld = nphysics2d::world::World::new();

        let mut specsWorld = specs::World::new();
        specsWorld.register::<crate::scene::physicstest::Ball>();
        specsWorld.register::<crate::system::Collider>();
        specsWorld.register::<crate::system::RigidBody>();
        specsWorld.register::<crate::system::Motion>();
        specsWorld.register::<crate::system::Mass>();
        specsWorld.register::<crate::system::Mesh>();
        specsWorld.register::<crate::system::Gravity>();

        specsWorld.add_resource(physicsWorld);

        crate::scene::physicstest::PhysicsTest::create_walls(context, &mut specsWorld);
        crate::scene::physicstest::PhysicsTest::create_ball(context, &mut specsWorld);
        crate::scene::physicstest::PhysicsTest::create_ground(context, &mut specsWorld);

        SceneWorld {
            specs: specsWorld,
            input: GInputState::new(),
            quit: false,
        }
    }
}

