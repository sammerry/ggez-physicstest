use ggez::graphics::DrawParam;
use ggez_goodies::scene::{Scene, SceneSwitch};

use nphysics2d::volumetric::Volumetric;

use specs;
use specs::prelude::*;

use log::*;

use crate::game::{Axis, Button, InputEvent};
use crate::world::SceneWorld;

const GRAVITY: f32 = 2.0;

// Restitution:
// 0.9 bouncy ball
// 0.5 baloon
// 0.0 wet toilet paper
const BALL_RESTITUTION: f32 = 0.0;

// Friction
// 50.0 velcro
// 2.0 rocks
// 0.0 ice
const BALL_FRICTION: f32 = 50.0;
const BALL_DENSITY: f32 = 1.0;

const CAMERA_WIDTH: f32 = 800.0;
const CAMERA_HEIGHT: f32 = 600.0;

const WALL_SIZE: f32 = 200.0;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Ball {
    active: bool,
}

pub struct PhysicsTest {
    dispatcher: specs::Dispatcher<'static, 'static>,
    drawBall: bool,
    gravity: bool,
    done: bool,
}

pub struct ActiveBall {
    pub object_handle: Option<ncollide2d::world::CollisionObjectHandle>,
}

impl PhysicsTest {
    pub fn new () -> PhysicsTest {
        debug!("Create CollisionTest");

        let physics = crate::system::PhysicsSystem{};
        let dispatcher = specs::DispatcherBuilder::new()
            .with(physics, "sys_physics", &[])
            .build();

        PhysicsTest {
            dispatcher: dispatcher,
            drawBall: false,
            gravity: false,
            done: false,
        }
    }



    /**
     *
     * Creates an entity as a collection of features including graphical
     * representation. Adds that entity to the physics world, and collider.
     * Then writes the entity coupled with the physics world handler.
     *
     */
    pub fn create_ball(context: &mut ggez::Context, specsWorld: &mut specs::World) {
        debug!("Create Ball");
        const RADIUS: f32 = 10.0;

        let entity = specsWorld.create_entity()
            .with(Ball { active: false })
            .with(crate::system::Motion {
                velocity: nalgebra::Vector2::new(1.5, -1.0),
                acceleration: nalgebra::Vector2::new(0.0, 0.0),
            })
            .with(crate::system::Mass { total: 0.0 })
            .with(crate::system::Gravity {
                force: None,
            })
            .with(crate::system::Mesh {
                mesh: ggez::graphics::MeshBuilder::default()
                    .circle(
                        ggez::graphics::DrawMode::Fill,
                        ggez::graphics::Point2::new(0.0, 0.0),
                        RADIUS,
                        0.1,
                    ).build(context)
                    .unwrap(),
            }).build();

        // Pull the physics world and add a shape
        let mut physics = specsWorld.write_resource::<crate::world::PhysicsWorld>();
        let shape = ncollide2d::shape::ShapeHandle::new(ncollide2d::shape::Ball::new(RADIUS));
        let position = nalgebra::Isometry2::new(nalgebra::Vector2::new(0.0, 0.0), 0.0);
        let inertia = shape.inertia(BALL_DENSITY);
        let centerOfMass = shape.center_of_mass();
        let bodyHandle = physics.add_rigid_body(position, inertia, centerOfMass);

        // Add the rigid body with status from shape
        let mut rigidBody = physics.rigid_body_mut(bodyHandle).unwrap();
        rigidBody.set_status(nphysics2d::object::BodyStatus::Dynamic);
        rigidBody.activation_status_mut().set_deactivation_threshold(None);

        // Add the rigid body to the collider
        let collideHandle = physics.add_collider(
            0.0,
            shape,
            bodyHandle,
            nalgebra::one(),
            nphysics2d::object::Material::new(BALL_RESTITUTION, BALL_FRICTION),
        );

        specsWorld
            .write_storage::<crate::system::Collider>()
            .insert(entity, crate::system::Collider {
                object_handle: collideHandle
            });

         specsWorld
            .write_storage::<crate::system::RigidBody>()
            .insert(entity, crate::system::RigidBody {
                object_handle: bodyHandle
            });
    }



    pub fn create_walls(context: &mut ggez::Context, specsWorld: &mut specs::World) {

        let entity = specsWorld
            .create_entity()
            .with(crate::system::Mesh {
                mesh: ggez::graphics::MeshBuilder::default()
                    .polygon(
                        ggez::graphics::DrawMode::Line(2.0),
                        &[
                            ggez::graphics::Point2::new(- WALL_SIZE,   WALL_SIZE),
                            ggez::graphics::Point2::new(- WALL_SIZE, - WALL_SIZE),
                            ggez::graphics::Point2::new(  WALL_SIZE, - WALL_SIZE),
                            ggez::graphics::Point2::new(  WALL_SIZE,   WALL_SIZE),
                        ],
                    )
                    .build(context).unwrap(),
            })
            .build();

        let mut physics = specsWorld.write_resource::<crate::world::PhysicsWorld>();

        let shape = ncollide2d::shape::ShapeHandle::new(
            ncollide2d::shape::Polyline::new(vec![
                nalgebra::Point2::new(- WALL_SIZE,   WALL_SIZE),
                nalgebra::Point2::new(- WALL_SIZE, - WALL_SIZE),
                nalgebra::Point2::new(  WALL_SIZE, - WALL_SIZE),
                nalgebra::Point2::new(  WALL_SIZE,   WALL_SIZE),
            ])
        );

        let position = nalgebra::one();
        let inertia =  nphysics2d::math::Inertia::zero();
        let center_of_mass = nalgebra::Point2::new(0.0, 0.0);
        let body_status = nphysics2d::object::BodyStatus::Static;
        let body_handle = physics.add_rigid_body(position, inertia, center_of_mass);
        let mut rigid_body = physics.rigid_body_mut(body_handle).unwrap();

        rigid_body
            .set_status(body_status);

        rigid_body
            .activation_status_mut()
            .set_deactivation_threshold(None);

        // bodies_handle.insert(entity, RigidBody(body_handle));

        let collideHandle = physics.add_collider(
            0.0,
            shape,
            body_handle,
            nalgebra::one(),
            nphysics2d::object::Material::new(BALL_RESTITUTION, BALL_FRICTION),
        );

        let collider = crate::system::Collider { object_handle: collideHandle };

        specsWorld
            .write_storage::<crate::system::Collider>()
            .insert(entity, collider);
    }



    pub fn create_ground(context: &mut ggez::Context, specsWorld: &mut specs::World) {
        let entity = specsWorld.create_entity()
            .build();

        let shape = ncollide2d::shape::ShapeHandle::new(
            ncollide2d::shape::Segment::new(
                nalgebra::Point2::new(- WALL_SIZE, WALL_SIZE),
                nalgebra::Point2::new(  WALL_SIZE, WALL_SIZE),
            )
        );

        let mut physics = specsWorld.write_resource::<crate::world::PhysicsWorld>();
        physics.add_collider(
            0.0,
            shape,
            nphysics2d::object::BodyHandle::ground(),
            nalgebra::one(),
            nphysics2d::object::Material::new(BALL_RESTITUTION, BALL_FRICTION),
        );
    }
}

impl Scene<SceneWorld, InputEvent> for PhysicsTest {
    fn name(&self) -> &str {
        "Physics Gravity Test"
    }

    /**
     * The update method. Triggers dispatchers like the Gravity system
     * Then checks for an exit case.
     */
    fn update(&mut self, world: &mut SceneWorld) -> SceneSwitch<SceneWorld, InputEvent> {
        self.dispatcher.dispatch(&mut world.specs.res);

        // Cut, exit stage right
        if self.done {
            SceneSwitch::Pop
        } else {
            SceneSwitch::None
        }
    }


    /**
     * The draw mehtod should show all available objects.
     */
    fn draw(self: &mut Self, sceneWorld: &mut SceneWorld, context: &mut ggez::Context) -> ggez::GameResult<()> {
        let camera_focus_x: f32 = 0.0;
        let camera_focus_y: f32 = 0.0;

        let screen_rect = ggez::graphics::Rect {
            x: camera_focus_x - CAMERA_WIDTH / 2.0,
            y: camera_focus_y - CAMERA_HEIGHT / 2.0,
            w: CAMERA_WIDTH,
            h: CAMERA_HEIGHT,
        };

        ggez::graphics::set_screen_coordinates(context, screen_rect)?;

        if (self.drawBall) {
            self.drawBall = false;
            PhysicsTest::create_ball(context, &mut sceneWorld.specs);
        }

        let mesh = sceneWorld.specs.read_storage::<crate::system::Mesh>();
        let collider = sceneWorld.specs.read_storage::<crate::system::Collider>();
        let mut physics_world = sceneWorld.specs.read_resource::<crate::world::PhysicsWorld>();
        let mut ncollide_world = physics_world.collision_world();

        // draw a list of objects that both are colliders and have meshes
        for (c, m) in (&collider, &mesh).join() {
            let collision_object = ncollide_world
                .collision_object(c.object_handle)
                .expect("Invalid collision object; was it removed from ncollide but not specs?");

            let isometry = collision_object.position();
            let point = ggez::nalgebra::Point2::new(isometry.translation.vector.x, isometry.translation.vector.y);
            let angle = isometry.rotation.angle();

            let drawParam = ggez::graphics::DrawParam {
                dest: point,
                rotation: angle,
                color: Some(ggez::graphics::WHITE),
                ..ggez::graphics::DrawParam::default()
            };

            ggez::graphics::draw_ex(context, &m.mesh, drawParam);
        }



        Ok(())
    }


    /**
     * Handles all input events
     * TODO: Change this to a pattern match
     */
    fn input(&mut self, sceneWorld: &mut SceneWorld, _ev: InputEvent, _started: bool) {

        if sceneWorld.input.get_button_pressed(Button::Shoot) {
            let mut physics = sceneWorld.specs.write_resource::<crate::world::PhysicsWorld>();
            let mut collider = physics.collision_world_mut();

            let balls = sceneWorld.specs.read_storage::<Ball>();
            let colliders = sceneWorld.specs.read_storage::<crate::system::Collider>();
            let rigidBodies = sceneWorld.specs.read_storage::<crate::system::RigidBody>();

            for (b, c, r) in (&balls, &colliders, &rigidBodies).join() {
                let angular: f32 = 0.0;
                let linear = nalgebra::Vector2::new(1000.0, 1000.0);
                let force = nphysics2d::math::Force::linear(linear * 1000.0);
                let mut maybeRB = physics.rigid_body_mut(r.object_handle);

                match maybeRB {
                    Some(rb) => {
                        rb.apply_force(&force);
                    },

                    None => {}
                }
            }
        }

        if sceneWorld.input.get_button_pressed(Button::Gravity) {
            let mut physics = sceneWorld.specs.write_resource::<crate::world::PhysicsWorld>();
            match self.gravity {
                false => {
                    physics.set_gravity(nalgebra::Vector2::y() * 98.0);
                    self.gravity = true;
                },

                true => {
                    physics.set_gravity(nalgebra::Vector2::y() * 0.0);
                    self.gravity = false;
                }
            }
        }

        if sceneWorld.input.get_button_pressed(Button::Rotate) {
            self.drawBall = true;
        }

        if sceneWorld.input.get_button_pressed(Button::Quit) {
            self.done = true;
        }
    }
}

