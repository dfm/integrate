pub mod forces;
pub mod leapfrog;
pub mod math;
pub mod system;

pub use forces::Force;
pub use system::System;

#[cfg(feature = "f32")]
pub type Float = f32;
#[cfg(not(feature = "f32"))]
pub type Float = f64;

#[cfg(feature = "2d")]
pub type Vector = math::Vec2;
#[cfg(not(feature = "2d"))]
pub type Vector = math::Vec3;

pub trait Integrator {
    fn init(&self);
    fn step(&self, dt: Float, system: &mut system::System);
}
