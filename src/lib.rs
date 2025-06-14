use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion([f64; 4]);

impl Quaternion {
    pub fn new(w: f64, x: f64, y: f64, z: f64) -> Self {
        Quaternion([w, x, y, z])
    }

    fn magnitude_sq(&self) -> f64 {
        self.0.iter().map(|&e| e.powi(2)).sum::<f64>()
    }

    pub fn magnitude(&self) -> f64 {
        self.magnitude_sq().sqrt()
    }

    pub fn conjugate(&self) -> Self {
        Quaternion([self.0[0], -self.0[1], -self.0[2], -self.0[3]])
    }

    pub fn inverse(&self) -> Self {
        let ms = self.magnitude_sq();
        let mut conj = self.conjugate();
        conj.0.iter_mut().for_each(|e| *e /= ms);
        conj
    }
}

impl Add for Quaternion {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Quaternion([
            self.0[0] + other.0[0],
            self.0[1] + other.0[1],
            self.0[2] + other.0[2],
            self.0[3] + other.0[3],
        ])
    }
}

impl Sub for Quaternion {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Quaternion([
            self.0[0] - other.0[0],
            self.0[1] - other.0[1],
            self.0[2] - other.0[2],
            self.0[3] - other.0[3],
        ])
    }
}

impl Mul for Quaternion {
    type Output = Self;

    // Hamilton product
    fn mul(self, other: Self) -> Self {
        Quaternion([
            self.0[0] * other.0[0]
                - self.0[1] * other.0[1]
                - self.0[2] * other.0[2]
                - self.0[3] * other.0[3],
            self.0[0] * other.0[1] + self.0[1] * other.0[0] + self.0[2] * other.0[3]
                - self.0[3] * other.0[2],
            self.0[0] * other.0[2] - self.0[1] * other.0[3]
                + self.0[2] * other.0[0]
                + self.0[3] * other.0[1],
            self.0[0] * other.0[3] + self.0[1] * other.0[2] - self.0[2] * other.0[1]
                + self.0[3] * other.0[0],
        ])
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
