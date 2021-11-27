use crate::{
    system::{Body, Configuration, Workspace},
    Float, Force, Vector,
};

#[derive(Clone, Debug)]
pub struct Gravity {
    constant: Float,
    softening: Float,
    parallel: bool,
}

impl Gravity {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_parallel() -> Self {
        Self {
            parallel: true,
            ..Self::default()
        }
    }

    pub fn set_constant(&mut self, constant: Float) -> &mut Self {
        self.constant = constant;
        self
    }

    pub fn set_softening(&mut self, softening: Float) -> &mut Self {
        self.softening = softening;
        self
    }

    pub fn set_parallel(&mut self, parallel: bool) -> &mut Self {
        self.parallel = parallel;
        self
    }

    fn accumulate_accelerations_serial(
        &self,
        configuration: &Configuration,
        workspace: &mut Workspace,
    ) {
        configuration
            .bodies
            .iter()
            .zip(workspace.body_accelerations.iter_mut())
            .enumerate()
            .for_each(|(i, (body1, acc))| {
                configuration
                    .bodies
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .for_each(|(_, body2)| {
                        let delta = body2.coord.position - body1.coord.position;
                        let factor =
                            body2.mass * calc_gravity_factor(self.constant, self.softening, &delta);
                        acc.inplace_add_scaled(factor, &delta);
                    });
            });

        configuration
            .particles
            .iter()
            .zip(workspace.particle_accelerations.iter_mut())
            .for_each(|(particle, acc)| {
                for body in configuration.bodies.iter() {
                    let delta = body.coord.position - particle.coord.position;
                    let factor =
                        body.mass * calc_gravity_factor(self.constant, self.softening, &delta);
                    acc.inplace_add_scaled(factor, &delta);
                }
            });
    }

    fn accumulate_accelerations_parallel(
        &self,
        configuration: &Configuration,
        workspace: &mut Workspace,
    ) {
        // let acc: Vec<Vector> = configuration
        //     .bodies
        //     .iter()
        //     .enumerate()
        //     .map(|(i, bodyi)| {
        //         configuration
        //             .bodies
        //             .iter()
        //             .enumerate()
        //             .filter(|(j, _)| i != *j)
        //             .map(|(_, bodyj)| {
        //                 let delta = bodyj.coord.position - bodyi.coord.position;
        //                 let factor =
        //                     bodyj.mass * calc_gravity_factor(self.constant, self.softening, &delta);
        //                 delta * factor
        //             })
        //             .fold(Vector::zero(), |a, b| a + b)
        //     })
        //     .collect();

        // {
        //     acc.iter()
        //         .zip(configuration.bodies.iter_mut())
        //         .for_each(|(a, body)| body.coord.acceleration = *a)
        // }

        // let bodies = &configuration.bodies;
        // configuration.particles.iter_mut().for_each(|particle| {
        //     for body in bodies {
        //         let delta = body.coord.position - particle.coord.position;
        //         let factor = body.mass * calc_gravity_factor(self.constant, self.softening, &delta);
        //         particle
        //             .coord
        //             .acceleration
        //             .inplace_add_scaled(factor, &delta);
        //     }
        // });
    }
}

impl Default for Gravity {
    fn default() -> Self {
        Self {
            constant: 1.0,
            softening: 0.0,
            parallel: false,
        }
    }
}

impl Force for Gravity {
    fn accumulate_accelerations(&self, configuration: &Configuration, workspace: &mut Workspace) {
        if self.parallel {
            self.accumulate_accelerations_parallel(configuration, workspace);
        } else {
            self.accumulate_accelerations_serial(configuration, workspace);
        }
    }
}

fn calc_gravity_factor(constant: Float, softening: Float, delta: &Vector) -> Float {
    let r2 = delta.squared_norm() + softening;
    constant / (r2 * r2.sqrt())
}

// fn apply_pairwise_self_gravity(
//     constant: Float,
//     softening: Float,
//     bodies_and_accelerations: &mut [(&Body, &mut Vector)],
// ) {
//     if let Some(((first, acc1), others)) = bodies_and_accelerations.split_first_mut() {
//         let m1 = first.mass;
//         let x1 = first.coord.position;
//         for (other, acc2) in others.iter_mut() {
//             let m2 = other.mass;
//             let x2 = other.coord.position;
//             let delta = x1 - x2;
//             let factor = calc_gravity_factor(constant, softening, &delta);
//             acc1.inplace_add_scaled(-factor * m2, &delta);
//             acc2.inplace_add_scaled(factor * m1, &delta);
//         }
//         apply_pairwise_self_gravity(constant, softening, others);
//     }
// }
