use integrate::gravity::BasicGravity;
use integrate::leapfrog::Leapfrog;
use integrate::{Stepper, System, Vec3};

fn main() {
    let mut system = System::new();
    system.add_body(10.0, Vec3::zero(), Vec3::zero());
    system.add_body(1.0, Vec3::new(10.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    let stepper = Stepper::new(BasicGravity::new(0.0), Leapfrog::new(0.01));

    for _ in 1..100 {
        stepper.step(&mut system);
    }

    println!("{}", system.t);
    println!("{:?}", system.bodies[1]);
    // let gravity = BasicGravity::new(0.0);
    // gravity.calculate_acceleration(&mut system);
    // println!("{:?}", system.bodies[1]);
}
