use num_traits::{One, Zero};
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion([f64; 4]);

// impl fmt::Display for Quaternion {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let [w, x, y, z] = self.0;
//         write!(f, "{}{:+}i{:+}j{:+}k", w, x, y, z)
//     }
// }

impl fmt::Display for Quaternion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Threshold for zeroing small values

        // Round and clean up small values
        fn clean(x: f64) -> f64 {
            if x.abs() < Quaternion::EPS {
                0.0
            } else {
                (x * 10_000.0).round() / 10_000.0
            }
        }

        let w = clean(self.0[0]);
        let x = clean(self.0[1]);
        let y = clean(self.0[2]);
        let z = clean(self.0[3]);

        write!(f, "{} {:+}i {:+}j {:+}k", w, x, y, z)
    }
}

impl Quaternion {
    const EPS: f64 = 1e-13;
    pub const fn new(w: f64, x: f64, y: f64, z: f64) -> Self {
        Quaternion([w, x, y, z])
    }

    pub fn dot(&self) -> f64 {
        self.0.iter().map(|&e| e * e).sum::<f64>()
    }

    pub fn eq(&self, other: &Quaternion, eps: f64) -> bool {
        self.0
            .iter()
            .zip(other.0.iter())
            .all(|(a, b)| (a - b).abs() < eps)
    }

    pub fn magnitude(&self) -> f64 {
        self.dot().sqrt()
    }

    pub fn conjugate(&self) -> Self {
        Quaternion([self.0[0], -self.0[1], -self.0[2], -self.0[3]])
    }

    pub fn inverse(&self) -> Self {
        let ms = self.dot();
        self.conjugate() / ms
    }
}

impl Zero for Quaternion {
    fn zero() -> Self {
        Quaternion([0.0; 4])
    }

    fn is_zero(&self) -> bool {
        self.0.iter().all(|&x| x.abs() < Self::EPS)
    }
}

impl One for Quaternion {
    fn one() -> Self {
        Quaternion([1.0, 0.0, 0.0, 0.0])
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

impl Neg for Quaternion {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Quaternion([-self.0[0], -self.0[1], -self.0[2], -self.0[3]])
    }
}

impl Mul<f64> for Quaternion {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Quaternion(self.0.map(|e| e * rhs))
    }
}

impl Div<f64> for Quaternion {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Quaternion(self.0.map(|e| e / rhs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conjugate() {
        let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let qc = Quaternion::new(1.0, -2.0, -3.0, -4.0);
        assert!(q.conjugate().eq(&qc, Quaternion::EPS));
    }

    #[test]
    fn test_inverse() {
        let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        let qi = q.inverse();
        let c = q * qi;
        let r = Quaternion::new(1.0, 0.0, 0.0, 0.0);
        assert!(c.eq(&r, Quaternion::EPS));
    }

    #[test]
    fn test_abs_of_product() {
        let q1 = Quaternion::new(3.06, 1.0, 1.0, 2.0);
        let q2 = Quaternion::new(0.70, 3.0, -1.0, 2.0);
        let p1 = (q1 * q2).dot();
        let p2 = q1.dot() * q2.dot();
        assert!((p1 - p2).abs() < Quaternion::EPS);
    }
}
