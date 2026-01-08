// Typed wrappers to make frame/point operations safer in the DSL
#[derive(Clone, Debug, PartialEq)]
pub struct Frame(pub nalgebra::Matrix4<f64>);

#[derive(Clone, Debug, PartialEq)]
pub struct Point(pub nalgebra::Vector4<f64>);

impl Frame {
    pub fn inverse(&self) -> Option<Frame> {
        self.0.try_inverse().map(Frame)
    }

    /// Compose this frame with `other` returning `other * self` (same as previous `compose!(self, other)`)
    pub fn compose(&self, other: &Frame) -> Frame {
        Frame(other.0 * self.0)
    }

    /// Apply this frame transform to a homogenous `Point`.
    pub fn apply_to_point(&self, p: &Point) -> Point {
        Point(self.0 * &p.0)
    }

    /// Conjugate `self` by `other`: other * self * other^-1
    pub fn rotate_wrt(&self, other: &Frame) -> Frame {
        let inv = other.0.try_inverse().unwrap();
        Frame(other.0 * self.0 * inv)
    }

    /// Rotate this frame around its local Z axis by `angle_deg` degrees.
    pub fn rotated_by_z_degrees(&self, angle_deg: f64) -> Frame {
        let rad = angle_deg.to_radians();
        let c = rad.cos();
        let s = rad.sin();
        let R = nalgebra::Matrix4::new(
            c, -s, 0.0, 0.0,
            s,  c, 0.0, 0.0,
            0.0,0.0,1.0,0.0,
            0.0,0.0,0.0,1.0,
        );
        Frame(R * self.0)
    }
}

impl From<nalgebra::Matrix4<f64>> for Frame {
    fn from(m: nalgebra::Matrix4<f64>) -> Self { Frame(m) }
}

impl From<nalgebra::Vector4<f64>> for Point {
    fn from(v: nalgebra::Vector4<f64>) -> Self { Point(v) }
}
