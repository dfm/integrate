use integrate::gravity::BasicGravity;
use integrate::leapfrog::Leapfrog;
use integrate::{Stepper, System};

fn main() {
    let mut system = System::new();
    system.add_body(10.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    system.add_body(1.0, 10.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    let stepper = Stepper::new(BasicGravity::new(0.0), Leapfrog::new(0.01));

    for _ in 1..100 {
        stepper.step(&mut system);
    }
}
