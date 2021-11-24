use crate::{Float, Vec3};

#[derive(Copy, Clone, Debug)]
pub struct Coord {
    pub position: Vec3,
    pub velocity: Vec3,
}

#[derive(Clone, Debug)]
pub struct System {
    pub(crate) t: Float,

    pub(crate) body_masses: Vec<Float>,
    pub(crate) body_positions: Vec<Vec3>,
    pub(crate) body_velocities: Vec<Vec3>,
    pub(crate) body_accelerations: Vec<Vec3>,

    pub(crate) particle_positions: Vec<Vec3>,
    pub(crate) particle_velocities: Vec<Vec3>,
    pub(crate) particle_accelerations: Vec<Vec3>,
}

macro_rules! getter {
    ( $typ:ty, $name:tt ) => {
        pub fn $name(&self) -> &$typ {
            &self.$name
        }
    };
}

impl System {
    pub fn new() -> System {
        System {
            t: 0.0,
            body_masses: vec![],
            body_positions: vec![],
            body_velocities: vec![],
            body_accelerations: vec![],
            particle_positions: vec![],
            particle_velocities: vec![],
            particle_accelerations: vec![],
        }
    }

    pub fn current_time(&self) -> Float {
        self.t
    }

    pub fn num_bodies(&self) -> usize {
        self.body_masses.len()
    }

    pub fn num_particles(&self) -> usize {
        self.particle_positions.len()
    }

    pub fn add_body(&mut self, mass: Float, position: Vec3, velocity: Vec3) {
        self.body_masses.push(mass);
        self.body_positions.push(position);
        self.body_velocities.push(velocity);
        self.body_accelerations.push(Vec3::zero());
    }

    pub fn zero_acceleration(&mut self) {
        for acc in self.body_accelerations.iter_mut() {
            acc.set_zero();
        }
        for acc in self.particle_accelerations.iter_mut() {
            acc.set_zero();
        }
    }

    getter!(Vec<Float>, body_masses);
    getter!(Vec<Vec3>, body_positions);
    getter!(Vec<Vec3>, body_velocities);
    getter!(Vec<Vec3>, body_accelerations);
    getter!(Vec<Vec3>, particle_positions);
    getter!(Vec<Vec3>, particle_velocities);
    getter!(Vec<Vec3>, particle_accelerations);
}
