use std::ops::{Mul, Not};

use nalgebra::{Matrix3, Vector2, Vector3};

use crate::so2::SO2;

pub struct SE2 {
    t: Matrix3<f64>,
}

impl SE2 {
    pub fn new(rotation: SO2, translation: Vector2<f64>) -> Self {
        let mut t = Matrix3::identity();
        t.fixed_view_mut::<2, 2>(0, 0)
            .copy_from(&rotation.get_matrix());
        t.fixed_view_mut::<2, 1>(0, 2).copy_from(&translation);
        Self { t }
    }

    pub fn identity() -> Self {
        Self {
            t: Matrix3::identity(),
        }
    }

    pub fn get_so(&self) -> SO2 {
        let m = self.t.fixed_view::<2, 2>(0, 0).into_owned();
        SO2::from_matrix(m)
    }

    pub fn get_translation(&self) -> Vector2<f64> {
        Vector2::new(self.t[(0, 2)], self.t[(1, 2)])
    }
}

impl std::fmt::Debug for SE2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SE2 [ {}, {}, {}\n      {}, {}, {}\n      {}, {}, {} ]",
            self.t[(0, 0)],
            self.t[(0, 1)],
            self.t[(0, 2)],
            self.t[(1, 0)],
            self.t[(1, 1)],
            self.t[(1, 2)],
            self.t[(2, 0)],
            self.t[(2, 1)],
            self.t[(2, 2)]
        )
    }
}

fn to_vec3(v: Vector2<f64>) -> Vector3<f64> {
    Vector3::new(v[0], v[1], 1.0)
}

fn to_vec2(v: Vector3<f64>) -> Vector2<f64> {
    Vector2::new(v[0], v[1])
}

impl Mul<Vector2<f64>> for &SE2 {
    type Output = Vector2<f64>;

    fn mul(self, rhs: Vector2<f64>) -> Vector2<f64> {
        let v3 = to_vec3(rhs);
        let v3_result = self.t * v3;
        to_vec2(v3_result)
    }
}

impl Not for &SE2 {
    type Output = SE2;

    fn not(self) -> Self::Output {
        let r = self.get_so();
        let t = self.get_translation();
        let r_inv = !r;
        let t_inv = -(r_inv * t);
        SE2::new(r_inv, t_inv)
    }
}

mod tests {
    #[cfg(test)]
    use super::*;

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
        assert!((pose_inv.t - expected).norm() < 1e-10);
    }
}