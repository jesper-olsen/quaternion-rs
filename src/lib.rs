use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion {
    pub w: f64, // scalar part
    pub x: f64, // i component
    pub y: f64, // j component
    pub z: f64, // k component
}

impl Quaternion {
    pub fn new(w: f64, x: f64, y: f64, z: f64) -> Self {
        Quaternion { w, x, y, z }
    }

    pub fn magnitude(&self) -> f64 {
        (self.w.powi(2) + self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalise(&mut self) {
        self.w = 1.0;
        self.x /= self.w;
        self.y /= self.w;
        self.z /= self.w;
    }

    pub fn conjugate(&self) -> Self {
        Quaternion {
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn inverse(&self) -> Self {
        let mag_squared = self.w.powi(2) + self.x.powi(2) + self.y.powi(2) + self.z.powi(2);
        let conj = self.conjugate();
        Quaternion {
            w: conj.w / mag_squared,
            x: conj.x / mag_squared,
            y: conj.y / mag_squared,
            z: conj.z / mag_squared,
        }
    }
}

impl Add for Quaternion {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Quaternion {
            w: self.w + other.w,
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Quaternion {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Quaternion {
            w: self.w - other.w,
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Quaternion {
    type Output = Self;

    // Hamilton product
    fn mul(self, other: Self) -> Self {
        Quaternion {
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
            x: self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
            y: self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x,
            z: self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w,
        }
    }
}

impl Div for Quaternion {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        self * other.inverse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        assert!(q.conjugate() == Quaternion::new(1.0, -2.0, -3.0, -4.0));
    }
}
