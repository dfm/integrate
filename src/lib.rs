pub mod gravity;
pub mod leapfrog;
pub mod stepper;
pub mod system;

pub use gravity::Gravity;
pub use stepper::Stepper;
pub use system::System;

pub type Float = f64;

pub trait Integrator {
    fn init(&self);
    fn part1(&self, system: &mut system::System);
    fn part2(&self, system: &mut system::System);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
