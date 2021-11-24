use crate::{Float, Integrator, System};

#[derive(Clone, Debug)]
pub struct Leapfrog {
    dt: Float,
}

impl Leapfrog {
    pub fn new(dt: Float) -> Leapfrog {
        Leapfrog { dt }
    }
}

impl Integrator for Leapfrog {
    fn init(&self) {}

    fn part1(&self, system: &mut System) {
        let half_dt = 0.5 * self.dt;
        for body in system.bodies.iter_mut() {
            body.position[0] += half_dt * body.velocity[0];
            body.position[1] += half_dt * body.velocity[1];
            body.position[2] += half_dt * body.velocity[2];
        }
        system.t += half_dt;
    }

    fn part2(&self, system: &mut System) {
        let dt = self.dt;
        let half_dt = 0.5 * self.dt;
        for body in system.bodies.iter_mut() {
            body.velocity[0] += dt * body.acceleration[0];
            body.velocity[1] += dt * body.acceleration[1];
            body.velocity[2] += dt * body.acceleration[2];

            body.position[0] += half_dt * body.velocity[0];
            body.position[1] += half_dt * body.velocity[1];
            body.position[2] += half_dt * body.velocity[2];
        }
        system.t += half_dt;
    }
}
