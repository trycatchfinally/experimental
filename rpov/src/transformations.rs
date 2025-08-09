use crate::{
    Tuple4,
    matrices::{Matrix4, MatrixElement},
};

fn translation<T: MatrixElement>(x: T, y: T, z: T) -> Matrix4<T> {
    let zero = <T as From<f32>>::from(0.0);
    let one = <T as From<f32>>::from(1.0);
    let data: [[T; 4]; 4] = [
        [one, zero, zero, x],
        [zero, one, zero, y],
        [zero, zero, one, z],
        [zero, zero, zero, one],
    ];
    Matrix4::from(data)
}

fn scaling<T: MatrixElement>(x: T, y: T, z: T) -> Matrix4<T> {
    let zero = <T as From<f32>>::from(0.0);
    let one = <T as From<f32>>::from(1.0);
    let data: [[T; 4]; 4] = [
        [x, zero, zero, zero],
        [zero, y, zero, zero],
        [zero, zero, z, zero],
        [zero, zero, zero, one],
    ];
    Matrix4::from(data)
}

fn rotation_x<T: MatrixElement>(r: T) -> Matrix4<T> {
    let cos_r = r.cos();
    let sin_r = r.sin();
    let zero = <T as From<f32>>::from(0.0);
    let one = <T as From<f32>>::from(1.0);
    let data: [[T; 4]; 4] = [
        [one, zero, zero, zero],
        [zero, cos_r, -sin_r, zero],
        [zero, sin_r, cos_r, zero],
        [zero, zero, zero, one],
    ];
    Matrix4::from(data)
}

fn rotation_y<T: MatrixElement>(r: T) -> Matrix4<T> {
    let cos_r = r.cos();
    let sin_r = r.sin();
    let zero = <T as From<f32>>::from(0.0);
    let one = <T as From<f32>>::from(1.0);
    let data: [[T; 4]; 4] = [
        [cos_r, zero, sin_r, zero],
        [zero, one, zero, zero],
        [-sin_r, zero, cos_r, zero],
        [zero, zero, zero, one],
    ];
    Matrix4::from(data)
}

fn rotation_z<T: MatrixElement>(r: T) -> Matrix4<T> {
    let cos_r = r.cos();
    let sin_r = r.sin();
    let zero = <T as From<f32>>::from(0.0);
    let one = <T as From<f32>>::from(1.0);
    let data: [[T; 4]; 4] = [
        [cos_r, -sin_r, zero, zero],
        [sin_r, cos_r, zero, zero],
        [zero, zero, one, zero],
        [zero, zero, zero, one],
    ];
    Matrix4::from(data)
}

fn shearing<T: MatrixElement>(xy: T, xz: T, yx: T, yz: T, zx: T, zy: T) -> Matrix4<T> {
    let zero = <T as From<f32>>::from(0.0);
    let one = <T as From<f32>>::from(1.0);
    let data: [[T; 4]; 4] = [
        [one, xy, xz, zero],
        [yx, one, yz, zero],
        [zx, zy, one, zero],
        [zero, zero, zero, one],
    ];
    Matrix4::from(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrices::{Matrix, check};
    use crate::{check_tuple, make_vector, point, vector};

    // Scenario: Multiplying by a translation matrix
    //   Given transform ← translation(5, -3, 2)
    //     And p ← point(-3, 4, 5)
    //    Then transform * p = point(2, 1, 7)
    #[test]
    fn multiplying_by_a_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = point(-3.0, 4.0, 5.0);
        assert_eq!(transform * p, point(2.0, 1.0, 7.0));
    }

    // Scenario: Multiplying by the inverse of a translation matrix
    //   Given transform ← translation(5, -3, 2)
    //     And inv ← inverse(transform)
    //     And p ← point(-3, 4, 5)
    //    Then inv * p = point(-8, 7, 3)
    #[test]
    fn multiplying_by_the_inverse_of_a_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let inv = transform.inverse();
        let p = point(-3.0, 4.0, 5.0);
        assert_eq!(inv * p, point(-8.0, 7.0, 3.0));
    }

    // Scenario: Translation does not affect vectors
    //   Given transform ← translation(5, -3, 2)
    //     And v ← vector(-3, 4, 5)
    //    Then transform * v = v
    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = translation(5.0, -3.0, 2.0);
        let v = make_vector(-3.0, 4.0, 5.0);
        assert_eq!(transform * v, v);
    }

    // Scenario: A scaling matrix applied to a point
    //   Given transform ← scaling(2, 3, 4)
    //     And p ← point(-4, 6, 8)
    //    Then transform * p = point(-8, 18, 32)
    #[test]
    fn a_scaling_matrix_applied_to_a_point() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = point(-4.0, 6.0, 8.0);
        assert_eq!(transform * p, point(-8.0, 18.0, 32.0));
    }

    // Scenario: A scaling matrix applied to a vector
    //   Given transform ← scaling(2, 3, 4)
    //     And v ← vector(-4, 6, 8)
    //    Then transform * v = vector(-8, 18, 32)
    #[test]
    fn a_scaling_matrix_applied_to_a_vector() {
        let transform = scaling(2.0, 3.0, 4.0);
        let v = make_vector(-4.0, 6.0, 8.0);
        assert_eq!(transform * v, make_vector(-8.0, 18.0, 32.0));
    }

    // Scenario: Multiplying by the inverse of a scaling matrix
    //   Given transform ← scaling(2, 3, 4)
    //     And inv ← inverse(transform)
    //     And v ← vector(-4, 6, 8)
    //    Then inv * v = vector(-2, 2, 2)
    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let transform = scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse();
        let v = make_vector(-4.0, 6.0, 8.0);
        assert_eq!(inv * v, make_vector(-2.0, 2.0, 2.0));
    }

    // Scenario: Reflection is scaling by a negative value
    //   Given transform ← scaling(-1, 1, 1)
    //     And p ← point(2, 3, 4)
    //    Then transform * p = point(-2, 3, 4)
    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let transform = scaling(-1.0, 1.0, 1.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, point(-2.0, 3.0, 4.0));
    }

    // Scenario: Rotating a point around the x axis
    //   Given p ← point(0, 1, 0)
    //     And half_quarter ← rotation_x(π / 4)
    //     And full_quarter ← rotation_x(π / 2)
    //   Then half_quarter * p = point(0, √2/2, √2/2)
    //     And full_quarter * p = point(0, 0, 1)
    #[test]
    fn rotating_a_point_around_the_x_axis() {
        use std::f64::consts::PI;
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);
        check_tuple(
            half_quarter * p,
            point(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        check_tuple(full_quarter * p, point(0.0, 0.0, 1.0));
    }

    // Scenario: The inverse of an x-rotation rotates in the opposite direction
    //   Given p ← point(0, 1, 0)
    //     And half_quarter ← rotation_x(π / 4)
    //     And inv ← inverse(half_quarter)
    //   Then inv * p = point(0, √2/2, -√2/2)
    #[test]
    fn the_inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
        use std::f32::consts::PI;
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let inv = half_quarter.inverse();
        let expected = point(0.0_f32, 2.0_f32.sqrt() / 2.0_f32, -2.0_f32.sqrt() / 2.0_f32);
        check_tuple(inv * p, expected);
    }

    // Scenario: Rotating a point around the y axis
    //   Given p ← point(0, 0, 1)
    //     And half_quarter ← rotation_y(π / 4)
    //     And full_quarter ← rotation_y(π / 2)
    //   Then half_quarter * p = point(√2/2, 0, √2/2)
    //     And full_quarter * p = point(1, 0, 0)
    #[test]
    fn rotating_a_point_around_the_y_axis() {
        use std::f64::consts::PI;
        let p = point(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);
        check_tuple(
            half_quarter * p,
            point(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0),
        );
        check_tuple(full_quarter * p, point(1.0, 0.0, 0.0));
    }

    // Scenario: Rotating a point around the z axis
    //   Given p ← point(0, 1, 0)
    //     And half_quarter ← rotation_z(π / 4)
    //     And full_quarter ← rotation_z(π / 2)
    //   Then half_quarter * p = point(-√2/2, √2/2, 0)
    //     And full_quarter * p = point(-1, 0, 0)
    #[test]
    fn rotating_a_point_around_the_z_axis() {
        use std::f64::consts::PI;
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);
        check_tuple(
            half_quarter * p,
            point(-2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0),
        );
        check_tuple(full_quarter * p, point(-1.0, 0.0, 0.0));
    }

    // Scenario: A shearing transformation moves x in proportion to y
    //   Given transform ← shearing(1, 0, 0, 0, 0, 0)
    //     And p ← point(2, 3, 4)
    //   Then transform * p = point(5, 3, 4)
    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, point(5.0, 3.0, 4.0));
    }

    // Scenario: A shearing transformation moves x in proportion to z
    //   Given transform ← shearing(0, 1, 0, 0, 0, 0)
    //     And p ← point(2, 3, 4)
    //   Then transform * p = point(6, 3, 4)
    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, point(6.0, 3.0, 4.0));
    }

    // Scenario: A shearing transformation moves y in proportion to x
    //   Given transform ← shearing(0, 0, 1, 0, 0, 0)
    //     And p ← point(2, 3, 4)
    //   Then transform * p = point(2, 5, 4)
    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, point(2.0, 5.0, 4.0));
    }

    // Scenario: A shearing transformation moves y in proportion to z
    //   Given transform ← shearing(0, 0, 0, 1, 0, 0)
    //     And p ← point(2, 3, 4)
    //   Then transform * p = point(2, 7, 4)
    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, point(2.0, 7.0, 4.0));
    }

    // Scenario: A shearing transformation moves z in proportion to x
    //   Given transform ← shearing(0, 0, 0, 0, 1, 0)
    //     And p ← point(2, 3, 4)
    //   Then transform * p = point(2, 3, 6)
    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, point(2.0, 3.0, 6.0));
    }

    // Scenario: A shearing transformation moves z in proportion to y
    //   Given transform ← shearing(0, 0, 0, 0, 0, 1)
    //     And p ← point(2, 3, 4)
    //   Then transform * p = point(2, 3, 7)
    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, point(2.0, 3.0, 7.0));
    }

    // Scenario: Individual transformations are applied in sequence
    //   Given p ← point(1, 0, 1)
    //     And A ← rotation_x(π / 2)
    //     And B ← scaling(5, 5, 5)
    //     And C ← translation(10, 5, 7)
    //   # apply rotation first
    //   When p2 ← A * p
    //   Then p2 = point(1, -1, 0)
    //   # then apply scaling
    //   When p3 ← B * p2
    //   Then p3 = point(5, -5, 0)
    //   # then apply translation
    //   When p4 ← C * p3
    //   Then p4 = point(15, 0, 7)
    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        use std::f32::consts::PI;
        let p = point(1.0, 0.0, 1.0);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

        let p2 = a * p;
        check_tuple(p2, point(1.0, -1.0, 0.0));

        let p3 = b * p2;
        check_tuple(p3, point(5.0_f32, -5.0, 0.0));

        let p4 = c * p3;
        check_tuple(p4, point(15.0_f32, 0.0, 7.0));
    }

    // Scenario: Chained transformations must be applied in reverse order
    //   Given p ← point(1, 0, 1)
    //     And A ← rotation_x(π / 2)
    //     And B ← scaling(5, 5, 5)
    //     And C ← translation(10, 5, 7)
    //   When T ← C * B * A
    //   Then T * p = point(15, 0, 7)
    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order() {
        use std::f64::consts::PI;
        let p = point(1.0, 0.0, 1.0);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);
        let t = c * b * a;
        assert_eq!(t * p, point(15.0, 0.0, 7.0));
    }
}

#[cfg(false)]
mod not_yet_implemented {

    #[cfg(test)]
    fn view_transform<T: MatrixElement>(
        from: Tuple4<T>,
        to: Tuple4<T>,
        up: Tuple4<T>,
    ) -> Matrix4<T> {
        todo!("view_transform {} {} {}", from, to, up)
    }

    // Scenario: The transformation matrix for the default orientation
    //   Given from ← point(0, 0, 0)
    //     And to ← point(0, 0, -1)
    //     And up ← vector(0, 1, 0)
    //   When t ← view_transform(from, to, up)
    //   Then t = identity_matrix
    #[test]
    fn the_transformation_matrix_for_the_default_orientation() {
        let from = point(0.0, 0.0, 0.0);
        let to = point(0.0, 0.0, -1.0);
        let up = vector(0.0, 1.0, 0.0);
        let t = view_transform(from, to, up);
        assert_eq!(t, identity_matrix());
    }

    // Scenario: A view transformation matrix looking in positive z direction
    //   Given from ← point(0, 0, 0)
    //     And to ← point(0, 0, 1)
    //     And up ← vector(0, 1, 0)
    //   When t ← view_transform(from, to, up)
    //   Then t = scaling(-1, 1, -1)
    #[test]
    fn a_view_transformation_matrix_looking_in_positive_z_direction() {
        let from = point(0.0, 0.0, 0.0);
        let to = point(0.0, 0.0, 1.0);
        let up = vector(0.0, 1.0, 0.0);
        let t = view_transform(from, to, up);
        assert_eq!(t, scaling(-1.0, 1.0, -1.0));
    }

    // Scenario: The view transformation moves the world
    //   Given from ← point(0, 0, 8)
    //     And to ← point(0, 0, 0)
    //     And up ← vector(0, 1, 0)
    //   When t ← view_transform(from, to, up)
    //   Then t = translation(0, 0, -8)
    #[test]
    fn the_view_transformation_moves_the_world() {
        let from = point(0.0, 0.0, 8.0);
        let to = point(0.0, 0.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);
        let t = view_transform(from, to, up);
        assert_eq!(t, translation(0.0, 0.0, -8.0));
    }

    // Scenario: An arbitrary view transformation
    //   Given from ← point(1, 3, 2)
    //     And to ← point(4, -2, 8)
    //     And up ← vector(1, 1, 0)
    //   When t ← view_transform(from, to, up)
    //   Then t is the following 4x4 matrix:
    //       | -0.50709 | 0.50709 |  0.67612 | -2.36643 |
    //       |  0.76772 | 0.60609 |  0.12122 | -2.82843 |
    //       | -0.35857 | 0.59761 | -0.71714 |  0.00000 |
    //       |  0.00000 | 0.00000 |  0.00000 |  1.00000 |
    #[test]
    fn an_arbitrary_view_transformation() {
        let from = point(1.0, 3.0, 2.0);
        let to = point(4.0, -2.0, 8.0);
        let up = vector(1.0, 1.0, 0.0);
        let t = view_transform(from, to, up);
        // This test is not implemented because it requires a real matrix implementation
        // and floating point comparisons.
        let expected = Matrix::from([
            [-0.50709, 0.50709, 0.67612, -2.36643],
            [0.76772, 0.60609, 0.12122, -2.82843],
            [-0.35857, 0.59761, -0.71714, 0.00000],
            [0.00000, 0.00000, 0.00000, 1.00000],
        ]);
        check(t, expected);
    }
}
