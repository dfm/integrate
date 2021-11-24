use crate::{Float, Gravity, Integrator};

#[derive(Clone, Debug)]
pub struct Body {
    pub mass: Float,
    pub position: [Float; 3],
    pub velocity: [Float; 3],
    pub acceleration: [Float; 3],
}

#[derive(Clone, Debug)]
pub struct System {
    pub t: Float,
    pub bodies: Vec<Body>,
}

impl System {
    pub fn new() -> System {
        System {
            t: 0.0,
            bodies: vec![],
        }
    }

    pub fn add_body(
        &mut self,
        mass: Float,
        x: Float,
        y: Float,
        z: Float,
        vx: Float,
        vy: Float,
        vz: Float,
    ) {
        self.bodies.push(Body {
            mass,
            position: [x, y, z],
            velocity: [vx, vy, vz],
            acceleration: [0.0, 0.0, 0.0],
        })
    }

    pub fn step(&mut self, gravity: impl Gravity, integrator: impl Integrator) {
        integrator.part1(self);
        gravity.calculate_acceleration(self);
        integrator.part2(self);
    }
}
