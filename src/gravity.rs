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
        let mut acceleration: Vec<[Float; 3]> = Vec::with_capacity(system.bodies.len());
        for acc in acceleration.iter_mut() {
            acc[0] = 0.0;
            acc[1] = 0.0;
            acc[2] = 0.0;
        }
        for (i, body1) in system.bodies.iter().enumerate() {
            for (j, body2) in system.bodies.iter().skip(i + 1).enumerate() {
                let delta: [Float; 3] = [
                    body1.position[0] - body2.position[0],
                    body1.position[1] - body2.position[1],
                    body1.position[2] - body2.position[2],
                ];
                let r2 = delta[0] * delta[0]
                    + delta[1] * delta[1]
                    + delta[2] * delta[2]
                    + self.softening;
                let factor = GRAV / (r2 * r2.sqrt());
                let factor1 = factor * body1.mass;
                let factor2 = -factor * body2.mass;

                // Using unsafe blocks to avoid bounds checking; we know we're in bounds
                unsafe {
                    let acc = acceleration.get_unchecked_mut(i);
                    acc[0] += factor2 * delta[0];
                    acc[1] += factor2 * delta[1];
                    acc[2] += factor2 * delta[2];
                }
                unsafe {
                    let acc = acceleration.get_unchecked_mut(j);
                    acc[0] += factor1 * delta[0];
                    acc[1] += factor1 * delta[1];
                    acc[2] += factor1 * delta[2];
                }
            }
        }
    }
}
