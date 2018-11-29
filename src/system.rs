/**
 *
 * Physics System Implementation
 *
 * Credit where credit is due
 * https://github.com/thiolliere/airjump-multi/blob/f716d3a755b6c5767d894a2a41c3c2ca96c1f2ef/src/system.rs
 *
 * This contains various specs::Systems. Each system is handled as its own thread
 * with access to a join of related structs. They calculate some feature and return.
 *
 * Other uses are audio or rendering or netowrking.
 *
 */

use log::*;

use specs;
use specs::prelude::*;

use nphysics2d;
use ncollide2d;



/// Mesh
#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Mesh {
    pub mesh: ggez::graphics::Mesh,
}



/// NCollide collision object handle.
/// This also stores position and orientation info.
#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Collider {
    pub object_handle: ncollide2d::world::CollisionObjectHandle,
}



/// NCollide collision object handle.
/// This also stores position and orientation info.
#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct RigidBody {
    pub object_handle: nphysics2d::object::BodyHandle,
}



/// Allows controlling physics bodies by applying
/// force to a rigid body
#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct ForceGenerator {
    pub object_handle: nphysics2d::object::BodyHandle,
    pub center: Point2,
}



/// Objects without one won't get affected by the `Gravity` system.
#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Mass {
    pub total: f32,
}



/// Attractive objects
#[derive(Clone, Debug, Component)]
#[storage(HashMapStorage)]
pub struct Gravity {
    pub force: Option<f32>,
}



/// Movers and shakers
#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Motion {
    pub velocity: nalgebra::Vector2<f32>,
    pub acceleration: nalgebra::Vector2<f32>,
}



/**
 * NPhysics2D World as f32 for ggez compatibility
 */
pub type PhysicsWorld = nphysics2d::world::World<f32>;
pub type CollisionWorld = nphysics2d::world::CollisionWorld<f32>;




/**
 * Gravity system, for the attractive objects ;)
 *
 * This calculates acceloration towards a given object.
 */
pub type Point2 = nalgebra::Point2<f32>;
pub struct PhysicsSystem;

impl<'a> specs::System<'a> for PhysicsSystem {
    type SystemData = (
        specs::WriteStorage<'a, Motion>,
        specs::ReadStorage<'a,  Gravity>,
        specs::ReadStorage<'a,  Collider>,
        specs::ReadStorage<'a,  Mass>,
        specs::WriteExpect<'a, PhysicsWorld>,
    );

    fn run(&mut self, (mut motion, gravity, collider, mass, mut physics_world): Self::SystemData) {
        physics_world.step();

        for contact in physics_world.contact_events() {
        }

        for proximity in physics_world.proximity_events() {
        }
    }
}

