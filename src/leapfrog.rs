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

macro_rules! leapfrog_update {
    ( $eps:expr, $x:expr, $dx:expr ) => {
        for (a, b) in $x.iter_mut().zip($dx.iter()) {
            a.inplace_add_scaled($eps, &b);
        }
    };
}

impl Integrator for Leapfrog {
    fn init(&self) {}

    fn part1(&self, system: &mut System) {
        let half_dt = 0.5 * self.dt;

        leapfrog_update!(half_dt, system.body_positions, system.body_velocities);
        leapfrog_update!(
            half_dt,
            system.particle_positions,
            system.particle_velocities
        );

        system.t += half_dt;
    }

    fn part2(&self, system: &mut System) {
        let dt = self.dt;
        let half_dt = 0.5 * self.dt;

        leapfrog_update!(dt, system.body_velocities, system.body_accelerations);
        leapfrog_update!(
            dt,
            system.particle_velocities,
            system.particle_accelerations
        );

        leapfrog_update!(half_dt, system.body_positions, system.body_velocities);
        leapfrog_update!(
            half_dt,
            system.particle_positions,
            system.particle_velocities
        );

        system.t += half_dt;
    }
}
