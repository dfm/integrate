use crate::{Float, Integrator, System};

#[derive(Clone, Debug)]
pub struct Leapfrog;

impl Leapfrog {
    pub fn new() -> Self {
        Leapfrog
    }
}

impl Integrator for Leapfrog {
    fn init(&self) {}
    fn step(&self, dt: Float, system: &mut System) {
        let half_dt = 0.5 * dt;

        // Take a half step in the positions
        system
            .configuration
            .iter_coords_mut()
            .for_each(|coord| coord.position.inplace_add_scaled(half_dt, &coord.velocity));
        system.t += half_dt;

        // Apply the forces
        system.compute_accelerations();

        // Take the second step and synchronize
        system
            .configuration
            .iter_coords_mut()
            .zip(system.workspace.iter_accels_mut())
            .for_each(|(coord, acc)| {
                coord.velocity.inplace_add_scaled(dt, &acc);
                coord.position.inplace_add_scaled(half_dt, &coord.velocity);
            });
        system.t += half_dt;
    }
}
