use cgmath::*;
use enrs::entity::sparse_set::SparseSet;

#[derive(Copy, Clone)]
struct Transform(Matrix4<f32>);

#[derive(Copy, Clone)]
struct Position(Vector3<f32>);

#[derive(Copy, Clone)]
struct Rotation(Vector3<f32>);

#[derive(Copy, Clone)]
struct Velocity(Vector3<f32>);

pub struct Benchmark {
    transforms: SparseSet<u32, Matrix4<f32>>,
    positions: SparseSet<u32, Vector3<f32>>,
    rotations: SparseSet<u32, Vector3<f32>>,
    velocities: SparseSet<u32, Vector3<f32>>,
    entities: Vec<u32>,
}

impl Default for Benchmark {
    fn default() -> Self {
        Self {
            transforms: SparseSet::new(),
            positions: SparseSet::new(),
            rotations: SparseSet::new(),
            velocities: SparseSet::new(),
            entities: Vec::default(),
        }
    }
}

impl Benchmark {
    pub fn new() -> Self {
        let mut benchmark = Benchmark::default();
        benchmark.transforms.reserve(10_000);
        benchmark.positions.reserve(10_000);
        benchmark.rotations.reserve(10_000);
        benchmark.velocities.reserve(10_000);

        for e in 0..10_000 {
            benchmark.transforms.insert(e, Matrix4::from_scale(1.0));
            benchmark.positions.insert(e, Vector3::unit_x());
            benchmark.rotations.insert(e, Vector3::unit_x());
            benchmark.velocities.insert(e, Vector3::unit_x());
        }

        benchmark.entities = (0..10_000).collect();

        benchmark
    }

    pub fn run(&mut self) {
        for &e in self.entities.iter() {
            if let Some(idx) = self.velocities.index(e) {
                let velocity = self.velocities.values().skip(idx).next().unwrap();
                let position = self.positions.values_mut().skip(idx).next().unwrap();
                *position += *velocity;
            }
        }
    }
}
