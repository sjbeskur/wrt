use nalgebra::{Matrix2, Vector2};
use std::{
    fmt::Debug,
    ops::{Mul, Not},
};

#[derive(Clone, Copy)]
pub struct SO2 {
    m: Matrix2<f64>,
}

impl SO2 {
    pub fn new(theta: f64) -> Self {
        let c = theta.cos();
        let s = theta.sin();
        let m = Matrix2::new(c, -s, s, c);
        SO2 { m }
    }

    pub fn from_matrix(m: Matrix2<f64>) -> Self {
        SO2 { m }
    }

    pub fn identity() -> Self {
        SO2 {
            m: Matrix2::identity(),
        }
    }

    pub fn get_matrix(&self) -> Matrix2<f64> {
        self.m
    }
}

impl Debug for SO2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SO2 [ {}, {}\n      {}, {} ]",
            self.m[(0, 0)],
            self.m[(0, 1)],
            self.m[(1, 0)],
            self.m[(1, 1)]
        )
    }
}

impl Mul for SO2 {
    type Output = SO2;

    fn mul(self, rhs: SO2) -> Self::Output {
        SO2 { m: self.m * rhs.m }
    }
}

impl Mul<Vector2<f64>> for SO2 {
    type Output = Vector2<f64>;

    fn mul(self, rhs: Vector2<f64>) -> Self::Output {
        self.m * rhs
    }
}

impl Not for SO2 {
    type Output = SO2;

    fn not(self) -> SO2 {
        SO2 {
            m: self.m.transpose(),
        }
    }
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn test_so2_multiplication() {
        let so1 = SO2::new(std::f64::consts::FRAC_PI_2); // 90 degrees
        let so2 = SO2::new(std::f64::consts::FRAC_PI_2); // 90 degrees
        let so3 = so1 * so2;

        let expected = SO2::new(std::f64::consts::PI); // 180 degrees

        assert!((so3.m - expected.m).norm() < 1e-10);
    }

    #[test]
    fn test_so2_identity() {
        let so = SO2::new(0.0); // 0 degrees
        let identity = Matrix2::identity(); // Identity matrix  
        assert!((so.m - identity).norm() < 1e-10);
        println!("SO2 identity matrix:\n{:?}", so);
    }

    #[test]
    fn test_angle() {
        let so = SO2::new(0.3);
        assert!((so.m[(0, 0)] - 0.9553).abs() < 0.00005);
        assert!((so.m[(0, 1)] - -0.2955).abs() < 0.00005);
        assert!((so.m[(1, 0)] - 0.2955).abs() < 0.00005);
        assert!((so.m[(1, 1)] - 0.9553).abs() < 0.00005);
    }

    #[test]
    fn test_so2_inverse() {
        let so = SO2::new(0.3);
        let so_inv = !so;
        let identity = so * so_inv;
        let expected = SO2::identity();
        assert!((identity.m - expected.m).norm() < 1e-10);
    }
}