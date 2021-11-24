use crate::{Float, Force, System, Vec3};

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

fn calc_grav_factor(softening: Float, delta: &Vec3) -> Float {
    let r2 = delta.squared_norm() + softening;
    GRAV / (r2 * r2.sqrt())
}

impl Force for Gravity {
    fn calculate_acceleration(&self, system: &mut System) {
        // Note: we assume that the acclerations were _already_ zeroed

        // We need to compute the mutual accelerations in an unsafe block since we're updating the
        // bodies in place, and we want to avoid bounds checking
        let body_iter = system.body_masses.iter().zip(system.body_positions.iter());
        for (i, (m1, &x1)) in body_iter.clone().enumerate() {
            for (j, (m2, &x2)) in body_iter.clone().skip(i + 1).enumerate() {
                let delta = x1 - x2;
                let factor = calc_grav_factor(self.softening, &delta);
                let factor1 = factor * m1;
                let factor2 = -factor * m2;

                unsafe {
                    let acc = system.body_accelerations.get_unchecked_mut(i);
                    acc.inplace_add_scaled(factor2, &delta);
                }

                unsafe {
                    let acc = system.body_accelerations.get_unchecked_mut(i + 1 + j);
                    acc.inplace_add_scaled(factor1, &delta);
                }
            }
        }

        // Update the accelerations for the test particles
        for (&px, pa) in system
            .particle_positions
            .iter()
            .zip(system.particle_accelerations.iter_mut())
        {
            for (mass, &x) in body_iter.clone() {
                let delta = x - px;
                let factor = mass * calc_grav_factor(self.softening, &delta);
                pa.inplace_add_scaled(factor, &delta);
            }
        }
    }
}
