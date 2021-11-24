pub mod gravity;
pub mod leapfrog;
pub mod stepper;
pub mod system;
pub mod vec3;

pub use gravity::Gravity;
pub use stepper::Stepper;
pub use system::System;
pub use vec3::Vec3;

pub type Float = f64;

pub trait Integrator {
    fn init(&self);
    fn part1(&self, system: &mut system::System);
    fn part2(&self, system: &mut system::System);
}

pub trait Force {
    fn calculate_acceleration(&self, system: &mut System);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
