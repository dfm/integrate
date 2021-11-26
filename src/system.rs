use crate::{Float, Vec3};

#[derive(Copy, Clone, Debug)]
pub struct Coord {
    pub position: Vec3,
    pub velocity: Vec3,
}

#[derive(Copy, Clone, Debug)]
pub struct PhaseSpaceCoord {
    pub position: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
}

#[derive(Copy, Clone, Debug)]
pub struct Body {
    pub mass: Float,
    pub coord: PhaseSpaceCoord,
}

#[derive(Copy, Clone, Debug)]
pub struct Particle {
    pub coord: PhaseSpaceCoord,
}

#[derive(Clone, Debug)]
pub struct System {
    pub t: Float,

    pub bodies: Vec<Body>,
    pub particles: Vec<Particle>,
}

impl System {
    pub fn new() -> System {
        System {
            t: 0.0,
            bodies: vec![],
            particles: vec![],
        }
    }

    pub fn current_time(&self) -> Float {
        self.t
    }

    pub fn num_bodies(&self) -> usize {
        self.bodies.len()
    }

    pub fn num_particles(&self) -> usize {
        self.particles.len()
    }

    pub fn add_body(&mut self, mass: Float, position: Vec3, velocity: Vec3) {
        self.bodies.push(Body {
            mass,
            coord: PhaseSpaceCoord {
                position,
                velocity,
                acceleration: Vec3::zero(),
            },
        });
    }

    pub fn zero_acceleration(&mut self) {
        for body in self.bodies.iter_mut() {
            body.coord.acceleration.set_zero();
        }
        for particle in self.particles.iter_mut() {
            particle.coord.acceleration.set_zero();
        }
    }
}
