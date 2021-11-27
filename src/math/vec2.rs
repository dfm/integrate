use crate::Float;
use std::ops;

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Vec2([Float; 2]);

impl Vec2 {
    pub fn new(x: Float, y: Float) -> Self {
        Self([x, y])
    }

    pub fn zero() -> Self {
        Self([0.0, 0.0])
    }

    pub fn set_zero(&mut self) {
        self.0[0] = 0.0;
        self.0[1] = 0.0;
    }

    pub fn inplace_add_scaled(&mut self, scale: Float, other: &Self) {
        self.0[0] += scale * other.0[0];
        self.0[1] += scale * other.0[1];
    }

    pub fn squared_norm(&self) -> Float {
        self.0[0] * self.0[0] + self.0[1] * self.0[1]
    }
}

impl ops::Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self([self.0[0] + other.0[0], self.0[1] + other.0[1]])
    }
}

impl ops::Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self([self.0[0] - other.0[0], self.0[1] - other.0[1]])
    }
}

impl ops::Mul<Float> for Vec2 {
    type Output = Self;
    fn mul(self, scale: Float) -> Self {
        Self([scale * self.0[0], scale * self.0[1]])
    }
}
