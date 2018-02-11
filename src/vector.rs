extern crate rand;

use std::f64::*;
use std::ops::*;
use rand::*;

#[derive(Debug, Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector {
    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalize(self) -> Vector {
        let length = self.length();
        Vector {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length
        }
    }

    pub fn cross(self, other: Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }

    pub fn dot(self, other: Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn interpolate(a: Vector, b: Vector, amount: f64) -> Vector {
        a * (1.0 - amount) + b * amount
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn zero() -> Vector {
        Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }

    pub fn one() -> Vector {
        Vector {
            x: 1.0,
            y: 1.0,
            z: 1.0
        }
    }

    pub fn random() -> Vector {
        let mut rng = thread_rng();
        let theta = rng.gen_range(0.0, 2.0 * consts::PI);
        let z = rng.gen_range(-1.0, 1.0);
        Vector {
            x: theta.sin(),
            y: theta.cos(),
            z: z
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl Mul for Vector {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Vector {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        rhs * self
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Vector {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}
