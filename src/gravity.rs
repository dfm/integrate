use crate::{
    system::{Body, Configuration},
    Float, Force, Vector,
};

#[derive(Clone, Debug)]
pub struct Gravity {
    constant: Float,
    softening: Float,
}

impl Gravity {
    pub fn set_constant(&mut self, constant: Float) -> &mut Self {
        self.constant = constant;
        self
    }

    pub fn set_softening(&mut self, softening: Float) -> &mut Self {
        self.softening = softening;
        self
    }
}

impl Default for Gravity {
    fn default() -> Self {
        Self {
            constant: 1.0,
            softening: 0.0,
        }
    }
}

impl Force for Gravity {
    fn accumulate_accelerations(&self, configuration: &mut Configuration) {
        apply_pairwise_self_gravity(self.constant, self.softening, &mut configuration.bodies);

        for particle in configuration.particles.iter_mut() {
            for body in configuration.bodies.iter_mut() {
                let delta = body.coord.position - particle.coord.position;
                let factor = body.mass * calc_gravity_factor(self.constant, self.softening, &delta);
                particle
                    .coord
                    .acceleration
                    .inplace_add_scaled(factor, &delta);
            }
        }
    }
}

fn calc_gravity_factor(constant: Float, softening: Float, delta: &Vector) -> Float {
    let r2 = delta.squared_norm() + softening;
    constant / (r2 * r2.sqrt())
}

fn apply_pairwise_self_gravity(constant: Float, softening: Float, bodies: &mut [Body]) {
    if let Some((first, others)) = bodies.split_first_mut() {
        let m1 = first.mass;
        let x1 = first.coord.position;
        for other in others.iter_mut() {
            let m2 = other.mass;
            let x2 = other.coord.position;
            let delta = x1 - x2;
            let factor = calc_gravity_factor(constant, softening, &delta);
            first
                .coord
                .acceleration
                .inplace_add_scaled(-factor * m2, &delta);
            other
                .coord
                .acceleration
                .inplace_add_scaled(factor * m1, &delta);
        }
        apply_pairwise_self_gravity(constant, softening, others);
    }
}
