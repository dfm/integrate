use crate::Float;
use std::ops;

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Vec3([Float; 3]);

impl Vec3 {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Self([x, y, z])
    }

    pub fn zero() -> Self {
        Self([0.0, 0.0, 0.0])
    }

    pub fn set_zero(&mut self) {
        self.0[0] = 0.0;
        self.0[1] = 0.0;
        self.0[2] = 0.0;
    }

    pub fn inplace_add_scaled(&mut self, scale: Float, other: &Self) {
        self.0[0] += scale * other.0[0];
        self.0[1] += scale * other.0[1];
        self.0[2] += scale * other.0[2];
    }

    pub fn squared_norm(&self) -> Float {
        self.0[0] * self.0[0] + self.0[1] * self.0[1] + self.0[2] * self.0[2]
    }
}

impl ops::Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self([
            self.0[0] + other.0[0],
            self.0[1] + other.0[1],
            self.0[2] + other.0[2],
        ])
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self([
            self.0[0] - other.0[0],
            self.0[1] - other.0[1],
            self.0[2] - other.0[2],
        ])
    }
}
