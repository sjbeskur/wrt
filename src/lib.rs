pub mod macros;
pub mod types;
pub mod so2;


#[cfg(test)]
mod tests {
    use super::*;
    use types::{Frame, Point};

    #[test]
    #[allow(non_snake_case)]
    fn nalgebra_transform_macros_work() {
        use nalgebra::{Matrix4, Vector4};

        // Translation by (1,2,3)
        let tx = 1.0_f64;
        let ty = 2.0_f64;
        let tz = 3.0_f64;
        let T = Frame::from(Matrix4::new(
            1.0, 0.0, 0.0, tx,
            0.0, 1.0, 0.0, ty,
            0.0, 0.0, 1.0, tz,
            0.0, 0.0, 0.0, 1.0,
        ));

        let p = Point::from(Vector4::new(0.5, -1.0, 2.0, 1.0));

        // apply_transform!: T.apply_to_point(p)
        let p_world = crate::apply_transform!(T, p.clone());
        assert_eq!(p_world, T.apply_to_point(&p));

        // compose!: create two translations and compose them
        let A = Frame::from(Matrix4::new(
            1.0, 0.0, 0.0, 1.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ));
        let B = Frame::from(Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 2.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ));

        let C = crate::compose!(A.clone(), B.clone());
        assert_eq!(C, B.compose(&A));

        // to_frame!: convert p from frame A to frame B given transforms to world
        // T_a_world translates by (1,0,0); T_b_world translates by (0,2,0)
        let T_a_world = Frame::from(Matrix4::new(
            1.0, 0.0, 0.0, 1.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ));
        let T_b_world = Frame::from(Matrix4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 2.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ));

        let p_a = Point::from(Vector4::new(0.0, 0.0, 0.0, 1.0));
        let p_b = crate::to_frame!(p_a.clone(), from: T_a_world.clone(), to: T_b_world.clone());

        // manual: world = T_a_world * p_a = (1,0,0,1) -> relative to B: inverse(T_b_world) * world
        let expected = T_b_world.inverse().unwrap().apply_to_point(&T_a_world.apply_to_point(&p_a));
        assert_eq!(p_b, expected);
    }

    #[test]
    #[allow(non_snake_case)]
    fn dsl_macros_work() {
        use nalgebra::Matrix4;

        // 90 degree rotation about Z
        let cos = 0.0_f64;
        let sin = 1.0_f64;
        let A = Frame::from(Matrix4::new(
            cos, -sin, 0.0, 0.0,
            sin,  cos, 0.0, 0.0,
            0.0,  0.0, 1.0, 0.0,
            0.0,  0.0, 0.0, 1.0,
        ));

        // 90 degree rotation about X
        let cos = 0.0_f64;
        let sin = 1.0_f64;
        let B = Frame::from(Matrix4::new(
            1.0, 0.0,  0.0, 0.0,
            0.0, cos, -sin, 0.0,
            0.0, sin,  cos, 0.0,
            0.0, 0.0,  0.0, 1.0,
        ));

        let r1 = crate::rotate!(A wrt B);
        assert_eq!(r1, A.rotate_wrt(&B));

        let r2 = crate::transform!(rotate A wrt B);
        assert_eq!(r2, A.rotate_wrt(&B));

        // angle form: rotate A by 90 degrees then conjugate wrt B
        let r3 = crate::rotate!(A , by 90.0 , wrt B);
        let expected_angle = A.rotated_by_z_degrees(90.0).rotate_wrt(&B);
        assert_eq!(r3, expected_angle);

        let r4 = crate::transform!(rotate A , by 90.0 , wrt B);
        assert_eq!(r4, expected_angle);
    }
}



