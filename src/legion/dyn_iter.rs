use cgmath::*;
use legion::*;
use query::Query;
use storage::PackOptions;

#[derive(Copy, Clone)]
struct Transform(Matrix4<f32>);

#[derive(Copy, Clone)]
struct Position(Vector3<f32>);

#[derive(Copy, Clone)]
struct Rotation(Vector3<f32>);

#[derive(Copy, Clone)]
struct Velocity(Vector3<f32>);

pub struct Benchmark(World, Query<(Read<Velocity>, Write<Position>)>, Vec<Entity>);

impl Benchmark {
    pub fn new() -> Self {
        let mut world = World::default();

        let entities: Vec<Entity> = world.extend(
            (
                vec![Transform(Matrix4::from_scale(1.0)); 10000],
                vec![Position(Vector3::unit_x()); 10000],
                vec![Rotation(Vector3::unit_x()); 10000],
                vec![Velocity(Vector3::unit_x()); 10000],
            )
                .into_soa(),
        ).into();
        world.pack(PackOptions::force());

        let query = <(Read<Velocity>, Write<Position>)>::query();

        Self(world, query, entities)
    }

    pub fn run(&mut self) {
        for &e in &self.2 {
            if let Some((velocity, position)) = self.1.get_mut(&mut self.0, e) {
                position.0 += velocity.0;
            }
        }
    }
}
