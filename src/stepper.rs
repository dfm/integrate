use crate::{Gravity, Integrator, System};

#[derive(Clone, Debug)]
pub struct Stepper<G: Gravity, I: Integrator> {
    pub gravity: G,
    pub integrator: I,
}

impl<G: Gravity, I: Integrator> Stepper<G, I> {
    pub fn new(gravity: G, integrator: I) -> Stepper<G, I> {
        Stepper {
            gravity,
            integrator,
        }
    }

    pub fn step(&self, system: &mut System) {
        self.integrator.part1(system);
        self.gravity.calculate_acceleration(system);
        self.integrator.part2(system);
    }
}
