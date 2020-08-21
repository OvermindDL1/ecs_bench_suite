use bevy_ecs::prelude::*;
use cgmath::*;

#[derive(Copy, Clone)]
struct Transform(Matrix4<f32>);

#[derive(Copy, Clone)]
struct Position(Vector3<f32>);

#[derive(Copy, Clone)]
struct Rotation(Vector3<f32>);

#[derive(Copy, Clone)]
struct Velocity(Vector3<f32>);

pub struct Benchmark(World, Vec<Entity>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::new();
        let entities: Vec<Entity> = world.spawn_batch((0..10_000).map(|_| {
            (
                Transform(Matrix4::from_scale(1.0)),
                Position(Vector3::unit_x()),
                Rotation(Vector3::unit_x()),
                Velocity(Vector3::unit_x()),
            )
        })).collect();

        Self(world, entities)
    }

    pub fn run(&mut self) {
        for &e in &self.1 {
            if let Ok(entity) = self.0.entity(e) {
                if let Some(velocity) = entity.get::<Velocity>() {
                    if let Some(mut position) = entity.get_mut::<Position>() {
                        position.0 += velocity.0
                    }
                }
            }
        }
    }
}
