use crate::{Float, Force, Vector};

#[derive(Copy, Clone, Debug)]
pub struct Coord {
    pub position: Vector,
    pub velocity: Vector,
    pub acceleration: Vector,
}

#[derive(Copy, Clone, Debug)]
pub struct Body {
    pub mass: Float,
    pub coord: Coord,
}

#[derive(Copy, Clone, Debug)]
pub struct Particle {
    pub coord: Coord,
}

#[derive(Clone, Debug)]
pub struct Configuration {
    pub bodies: Vec<Body>,
    pub particles: Vec<Particle>,
}

impl Configuration {
    pub fn new() -> Self {
        Self {
            bodies: vec![],
            particles: vec![],
        }
    }

    pub fn add_body(&mut self, mass: Float, position: Vector, velocity: Vector) -> &mut Self {
        self.bodies.push(Body {
            mass,
            coord: Coord {
                position,
                velocity,
                acceleration: Vector::zero(),
            },
        });
        self
    }

    pub fn zero_accelerations(&mut self) {
        for body in self.bodies.iter_mut() {
            body.coord.acceleration.set_zero();
        }
        for particle in self.particles.iter_mut() {
            particle.coord.acceleration.set_zero();
        }
    }

    pub fn iter_coords_mut(&mut self) -> impl Iterator<Item = &mut Coord> {
        let body_coords = self.bodies.iter_mut().map(|body| &mut body.coord);
        let particle_coords = self
            .particles
            .iter_mut()
            .map(|particle| &mut particle.coord);
        body_coords.chain(particle_coords)
    }
}

pub struct System {
    pub t: Float,
    pub configuration: Configuration,
    pub forces: Vec<Box<dyn Force>>,
}

impl System {
    pub fn new() -> Self {
        Self {
            t: 0.0,
            configuration: Configuration::new(),
            forces: vec![],
        }
    }

    pub fn current_time(&self) -> Float {
        self.t
    }

    pub fn add_body(&mut self, mass: Float, position: Vector, velocity: Vector) -> &mut Self {
        self.configuration.add_body(mass, position, velocity);
        self
    }

    pub fn add_force(&mut self, force: Box<dyn Force>) -> &mut Self {
        self.forces.push(force);
        self
    }

    pub fn compute_accelerations(&mut self) {
        self.configuration.zero_accelerations();
        for force in &self.forces {
            force.accumulate_accelerations(&mut self.configuration);
        }
    }
}
