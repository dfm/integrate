use integrate::gravity::Gravity;
use integrate::leapfrog::Leapfrog;
use integrate::{Stepper, System, Vec3};

fn main() {
    let mut system = System::new();
    system.add_body(10.0, Vec3::zero(), Vec3::zero());
    system.add_body(1.0, Vec3::new(10.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));

    let mut stepper = Stepper::new(Leapfrog::new(0.01));
    stepper.add_force(Box::new(Gravity::new(0.0)));

    for _ in 1..100 {
        stepper.step(&mut system);
    }

    println!("{}", system.current_time());
    println!("{:?}", system.bodies[1]);
}
