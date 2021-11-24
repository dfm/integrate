use crate::{Float, Gravity, Integrator, Vec3};

#[derive(Copy, Clone, Debug)]
pub struct Body {
    pub mass: Float,
    pub position: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
}

#[derive(Copy, Clone, Debug)]
pub struct Particle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
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

    pub fn add_body(&mut self, mass: Float, position: Vec3, velocity: Vec3) {
        self.bodies.push(Body {
            mass,
            position,
            velocity,
            acceleration: Vec3::zero(),
        })
    }

    pub fn step(&mut self, gravity: impl Gravity, integrator: impl Integrator) {
        integrator.part1(self);
        gravity.calculate_acceleration(self);
        integrator.part2(self);
    }
}
