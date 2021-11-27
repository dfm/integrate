use crate::{Float, Force, Vector};

#[derive(Copy, Clone, Debug)]
pub struct Coord {
    pub position: Vector,
    pub velocity: Vector,
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

#[derive(Clone, Debug)]
pub struct Workspace {
    pub body_accelerations: Vec<Vector>,
    pub particle_accelerations: Vec<Vector>,
}

impl Workspace {
    pub fn new() -> Self {
        Self {
            body_accelerations: vec![],
            particle_accelerations: vec![],
        }
    }

    pub fn resize(&mut self, num_bodies: usize, num_particles: usize) {
        self.body_accelerations.resize(num_bodies, Vector::zero());
        self.particle_accelerations
            .resize(num_particles, Vector::zero());
    }

    pub fn set_zero(&mut self) {
        for acc in self
            .body_accelerations
            .iter_mut()
            .chain(self.particle_accelerations.iter_mut())
        {
            acc.set_zero();
        }
    }

    pub fn iter_accels_mut(&mut self) -> impl Iterator<Item = &mut Vector> {
        let body_accels = self.body_accelerations.iter_mut();
        let particle_accels = self.particle_accelerations.iter_mut();
        body_accels.chain(particle_accels)
    }
}

impl Configuration {
    pub fn new() -> Self {
        Self {
            bodies: vec![],
            particles: vec![],
        }
    }

    pub fn num_bodies(&self) -> usize {
        self.bodies.len()
    }

    pub fn num_particles(&self) -> usize {
        self.particles.len()
    }

    pub fn add_body(&mut self, mass: Float, position: Vector, velocity: Vector) -> &mut Self {
        self.bodies.push(Body {
            mass,
            coord: Coord { position, velocity },
        });
        self
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
    pub workspace: Workspace,
    pub forces: Vec<Box<dyn Force>>,
}

impl System {
    pub fn new() -> Self {
        Self {
            t: 0.0,
            configuration: Configuration::new(),
            workspace: Workspace::new(),
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

    pub fn zero_accelerations(&mut self) {
        self.workspace.resize(
            self.configuration.num_bodies(),
            self.configuration.num_particles(),
        );
        self.workspace.set_zero();
    }

    pub fn compute_accelerations(&mut self) {
        self.zero_accelerations();
        for force in &self.forces {
            force.accumulate_accelerations(&self.configuration, &mut self.workspace);
        }
    }
}
