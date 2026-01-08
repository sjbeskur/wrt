// Clean single-definition SE2 using Isometry2
use std::ops::{Mul, Not};

use nalgebra::{Isometry2, Matrix3, Vector2};

use crate::so2::SO2;

#[derive(Clone)]
pub struct SE2 {
    iso: Isometry2<f64>,
}

impl SE2 {
    pub fn new(rotation: SO2, translation: Vector2<f64>) -> Self {
        let angle = rotation.angle();
        let iso = Isometry2::new(translation, angle);
        Self { iso }
    }

    pub fn identity() -> Self {
        Self {
            iso: Isometry2::identity(),
        }
    }

    pub fn get_so(&self) -> SO2 {
        SO2::new(self.iso.rotation.angle())
    }

    pub fn get_translation(&self) -> Vector2<f64> {
        self.iso.translation.vector
    }

    /// Compose this transform with another: self * other (isometry composition)
    pub fn compose(&self, other: &SE2) -> SE2 {
        SE2 {
            iso: self.iso * other.iso,
        }
    }

    /// Apply this SE2 transform to a 2D point.
    pub fn apply_to_point(&self, p: &Vector2<f64>) -> Vector2<f64> {
        self.iso * p
    }
}

impl std::fmt::Debug for SE2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let m: Matrix3<f64> = self.iso.to_homogeneous();
        write!(
            f,
            "SE2 [ {}, {}, {}\n      {}, {}, {}\n      {}, {}, {} ]",
            m[(0, 0)], m[(0, 1)], m[(0, 2)], m[(1, 0)], m[(1, 1)], m[(1, 2)], m[(2, 0)], m[(2, 1)], m[(2, 2)]
        )
    }
}

impl Mul<Vector2<f64>> for &SE2 {
    type Output = Vector2<f64>;

    fn mul(self, rhs: Vector2<f64>) -> Vector2<f64> {
        self.apply_to_point(&rhs)
    }
}

impl Not for &SE2 {
    type Output = SE2;

    fn not(self) -> Self::Output {
        SE2 {
            iso: self.iso.inverse(),
        }
    }
}

impl Mul for SE2 {
    type Output = SE2;

    fn mul(self, rhs: SE2) -> SE2 {
        SE2 { iso: self.iso * rhs.iso }
    }
}

impl Mul<&SE2> for &SE2 {
    type Output = SE2;

    fn mul(self, rhs: &SE2) -> SE2 {
        SE2 { iso: self.iso * rhs.iso }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Matrix3;

    #[test]
    fn test_se2_identity() {
        let se = SE2::identity();
        let point = Vector2::new(1.0, 2.0);
        let transformed_point = &se * point;
        assert!((transformed_point - point).norm() < 1e-10);
    }

    #[test]
    fn test_se2_inverse() {
        let pose = SE2::new(SO2::identity(), Vector2::new(4.0, 2.0));
        let pose_inv = !&pose;
        let expected = Matrix3::new(1.0, 0.0, -4.0, 0.0, 1.0, -2.0, 0.0, 0.0, 1.0);
        let m: Matrix3<f64> = pose_inv.iso.to_homogeneous();
        assert!((m - expected).norm() < 1e-10);
    }

    #[test]
    fn test_se2_compose_and_apply() {
        let t1 = SE2::new(SO2::identity(), Vector2::new(1.0, 0.0));
        let t2 = SE2::new(SO2::identity(), Vector2::new(0.0, 2.0));

        // compose via Mul without cloning
        let comp = &t1 * &t2;
        let expected = SE2::new(SO2::identity(), Vector2::new(1.0, 2.0));
        let m_comp: Matrix3<f64> = comp.iso.to_homogeneous();
        let m_expected: Matrix3<f64> = expected.iso.to_homogeneous();
        assert!((m_comp - m_expected).norm() < 1e-10);

        // apply_to_point should match &SE2 * point
        let point = Vector2::new(3.0, 4.0);
        let via_mul = &t1 * point;
        let via_method = t1.apply_to_point(&Vector2::new(3.0, 4.0));
        assert!((via_mul - via_method).norm() < 1e-10);
    }
}