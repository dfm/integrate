use crate::{Force, Integrator, System};

pub struct Stepper<I: Integrator> {
    integrator: I,
    forces: Vec<Box<dyn Force>>,
}

impl<I: Integrator> Stepper<I> {
    pub fn new(integrator: I) -> Stepper<I> {
        Stepper {
            integrator,
            forces: vec![],
        }
    }

    pub fn add_force(&mut self, force: Box<dyn Force>) {
        self.forces.push(force);
    }

    pub fn step(&self, system: &mut System) {
        system.zero_acceleration();
        self.integrator.part1(system);
        for force in self.forces.iter() {
            force.calculate_acceleration(system);
        }
        self.integrator.part2(system);
    }
}
