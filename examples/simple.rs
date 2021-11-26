use integrate::gravity::Gravity;
use integrate::leapfrog::Leapfrog;
use integrate::{Integrator, System, Vector};

fn main() {
    let integrator = Leapfrog::new();
    let mut system = System::new();
    system.add_body(10.0, Vector::zero(), Vector::zero());
    system.add_body(1.0, Vector::new(10.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
    system.add_force(Box::new(Gravity::default()));

    for _ in 1..100 {
        integrator.step(0.01, &mut system);
    }

    println!("{}", system.current_time());
    println!("{:?}", system.configuration.bodies[1]);
}
