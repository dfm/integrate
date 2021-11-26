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
        macro_rules! leapfrog_update1 {
            ( $target:expr, $eps:expr ) => {
                $target.iter_mut().for_each(|target| {
                    target
                        .coord
                        .position
                        .inplace_add_scaled($eps, &target.coord.velocity)
                });
            };
        }

        let half_dt = 0.5 * self.dt;
        leapfrog_update1!(system.bodies, half_dt);
        leapfrog_update1!(system.particles, half_dt);
        system.t += half_dt;
    }

    fn part2(&self, system: &mut System) {
        macro_rules! leapfrog_update2 {
            ( $target:expr, $eps:expr, $half_eps:expr ) => {
                $target.iter_mut().for_each(|target| {
                    target
                        .coord
                        .velocity
                        .inplace_add_scaled($eps, &target.coord.acceleration);
                    target
                        .coord
                        .position
                        .inplace_add_scaled($half_eps, &target.coord.velocity)
                });
            };
        }

        let dt = self.dt;
        let half_dt = 0.5 * self.dt;
        leapfrog_update2!(system.bodies, dt, half_dt);
        leapfrog_update2!(system.particles, dt, half_dt);
        system.t += half_dt;
    }
}
