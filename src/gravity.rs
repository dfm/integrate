use crate::{Float, System};

const GRAV: Float = 1.0;

pub trait Gravity {
    fn calculate_acceleration(&self, system: &mut System);
}

#[derive(Clone, Debug)]
pub struct BasicGravity {
    softening: Float,
}

impl BasicGravity {
    pub fn new(softening: Float) -> BasicGravity {
        BasicGravity { softening }
    }
}

impl Gravity for BasicGravity {
    fn calculate_acceleration(&self, system: &mut System) {
        // First zero out all the accelerations
        for body in system.bodies.iter_mut() {
            body.acceleration.set_zero();
        }

        // We need to compute the mutual accelerations in an unsafe block since we're updating the
        // bodies in place, and we want to avoid bounds checking
        let num_bodies = system.bodies.len();
        unsafe {
            for i in 0..num_bodies {
                for j in (i + 1)..num_bodies {
                    let (delta, factor1, factor2) = {
                        let body1 = system.bodies.get_unchecked(i);
                        let body2 = system.bodies.get_unchecked(j);
                        let delta = body1.position - body2.position;
                        let r2 = delta.squared_norm() + self.softening;
                        let factor = GRAV / (r2 * r2.sqrt());
                        (delta, factor * body1.mass, -factor * body2.mass)
                    };

                    {
                        let body1 = system.bodies.get_unchecked_mut(i);
                        body1.acceleration.inplace_add_scaled(factor2, &delta);
                    }

                    {
                        let body2 = system.bodies.get_unchecked_mut(j);
                        body2.acceleration.inplace_add_scaled(factor1, &delta);
                    }
                }
            }
        }

        // Update the accelerations for the test particles
        for particle in system.particles.iter_mut() {
            particle.acceleration.set_zero();
            for body in system.bodies.iter() {
                let delta = body.position - particle.position;
                let r2 = delta.squared_norm() + self.softening;
                let factor = GRAV * body.mass / (r2 * r2.sqrt());
                particle.acceleration.inplace_add_scaled(factor, &delta);
            }
        }
    }
}
