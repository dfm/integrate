use crate::{system::Body, Float, Force, System, Vec3};

const GRAV: Float = 1.0;

#[derive(Clone, Debug)]
pub struct Gravity {
    softening: Float,
}

impl Gravity {
    pub fn new(softening: Float) -> Self {
        Self { softening }
    }
}

impl Force for Gravity {
    fn calculate_acceleration(&self, system: &mut System) {
        // Note: we assume that the acclerations were _already_ zeroed
        apply_pairwise_self_gravity(self.softening, &mut system.bodies);

        for particle in system.particles.iter_mut() {
            for body in system.bodies.iter_mut() {
                let delta = body.coord.position - particle.coord.position;
                let factor = body.mass * calc_gravity_factor(self.softening, &delta);
                particle
                    .coord
                    .acceleration
                    .inplace_add_scaled(factor, &delta);
            }
        }
    }
}

fn calc_gravity_factor(softening: Float, delta: &Vec3) -> Float {
    let r2 = delta.squared_norm() + softening;
    GRAV / (r2 * r2.sqrt())
}

fn apply_pairwise_self_gravity(softening: Float, bodies: &mut [Body]) {
    if let Some((first, others)) = bodies.split_first_mut() {
        let m1 = first.mass;
        let x1 = first.coord.position;
        for other in others.iter_mut() {
            let m2 = other.mass;
            let x2 = other.coord.position;
            let delta = x1 - x2;
            let factor = calc_gravity_factor(softening, &delta);
            first
                .coord
                .acceleration
                .inplace_add_scaled(-factor * m2, &delta);
            other
                .coord
                .acceleration
                .inplace_add_scaled(factor * m1, &delta);
        }
        apply_pairwise_self_gravity(softening, others);
    }
}
