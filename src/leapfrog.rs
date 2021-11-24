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
            body.position.inplace_add_scaled(half_dt, &body.velocity);
        }
        system.t += half_dt;
    }

    fn part2(&self, system: &mut System) {
        let dt = self.dt;
        let half_dt = 0.5 * self.dt;
        for body in system.bodies.iter_mut() {
            body.velocity.inplace_add_scaled(dt, &body.acceleration);
            body.position.inplace_add_scaled(half_dt, &body.velocity);
        }
        system.t += half_dt;
    }
}
