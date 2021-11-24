use crate::Float;
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    x: [Float; 3],
}

impl Vec3 {
    pub fn new(x: Float, y: Float, z: Float) -> Vec3 {
        Vec3 { x: [x, y, z] }
    }

    pub fn zero() -> Vec3 {
        Vec3 { x: [0.0, 0.0, 0.0] }
    }

    pub fn set_zero(&mut self) {
        self.x[0] = 0.0;
        self.x[1] = 0.0;
        self.x[2] = 0.0;
    }

    pub fn inplace_add_scaled(&mut self, scale: Float, other: &Vec3) {
        self.x[0] += scale * other.x[0];
        self.x[1] += scale * other.x[1];
        self.x[2] += scale * other.x[2];
    }

    pub fn squared_norm(&self) -> Float {
        self.x[0] * self.x[0] + self.x[1] * self.x[1] + self.x[2] * self.x[2]
    }
}

impl ops::Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: [
                self.x[0] + other.x[0],
                self.x[1] + other.x[1],
                self.x[2] + other.x[2],
            ],
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: [
                self.x[0] - other.x[0],
                self.x[1] - other.x[1],
                self.x[2] - other.x[2],
            ],
        }
    }
}
