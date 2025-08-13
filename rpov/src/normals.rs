#[cfg(test)]
mod tests {
    use crate::floats::consts::SQRT_2;

    use crate::floats::Float;
    use crate::floats::consts::{FRAC_1_SQRT_2, PI};
    use crate::spheres::Sphere;
    use crate::transformations::{rotation_z, scaling, translation};
    use crate::tuples::check_tuple;
    use crate::tuples::{point, vector};

    // Scenario: The normal on a sphere at a point on the x axis
    //   Given s ← sphere()
    //   When n ← normal_at(s, point(1, 0, 0))
    //   Then n = vector(1, 0, 0)
    #[test]
    fn test_normal_on_sphere_at_x_axis() {
        let s = Sphere::new();
        let n = s.normal_at(&point(1.0, 0.0, 0.0));
        assert_eq!(n, vector(1.0, 0.0, 0.0));
    }

    // Scenario: The normal on a sphere at a point on the y axis
    //   Given s ← sphere()
    //   When n ← normal_at(s, point(0, 1, 0))
    //   Then n = vector(0, 1, 0)
    #[test]
    fn test_normal_on_sphere_at_y_axis() {
        let s = Sphere::new();
        let n = s.normal_at(&point(0.0, 1.0, 0.0));
        assert_eq!(n, vector(0.0, 1.0, 0.0));
    }

    // Scenario: The normal on a sphere at a point on the z axis
    //   Given s ← sphere()
    //   When n ← normal_at(s, point(0, 0, 1))
    //   Then n = vector(0, 0, 1)
    #[test]
    fn test_normal_on_sphere_at_z_axis() {
        let s = Sphere::new();
        let n = s.normal_at(&point(0.0, 0.0, 1.0));
        assert_eq!(n, vector(0.0, 0.0, 1.0));
    }

    // Scenario: The normal on a sphere at a nonaxial point
    //   Given s ← sphere()
    //   When n ← normal_at(s, point(√3/3, √3/3, √3/3))
    //   Then n = vector(√3/3, √3/3, √3/3)
    #[test]
    fn test_normal_on_sphere_at_nonaxial_point() {
        let s = Sphere::new();
        let val = Float::from(3.0).sqrt() / 3.0;
        let n = s.normal_at(&point(val, val, val));
        check_tuple(n, vector(val, val, val));
    }

    // Scenario: The normal is a normalized vector
    //   Given s ← sphere()
    //   When n ← normal_at(s, point(√3/3, √3/3, √3/3))
    //   Then n = normalize(n)
    #[test]
    fn test_normal_is_normalized_vector() {
        let s = Sphere::new();
        let val = Float::from(3.0).sqrt() / 3.0;
        let n = s.normal_at(&point(val, val, val));
        check_tuple(n, n.normalize());
    }

    // Scenario: Computing the normal on a translated sphere
    //   Given s ← sphere()
    //     And set_transform(s, translation(0, 1, 0))
    //   When n ← normal_at(s, point(0, 1.70711, -0.70711))
    //   Then n = vector(0, 0.70711, -0.70711)
    #[test]
    fn test_computing_normal_on_translated_sphere() {
        let s = Sphere::with_transform(translation(0.0, 1.0, 0.0));
        let n = s.normal_at(&point(0.0, 1.0 + FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
        check_tuple(n, vector(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
    }

    // Scenario: Computing the normal on a transformed sphere
    //   Given s ← sphere()
    //     And m ← scaling(1, 0.5, 1) * rotation_z(π/5)
    //     And set_transform(s, m)
    //   When n ← normal_at(s, point(0, √2/2, -√2/2))
    //   Then n = vector(0, 0.97014, -0.24254)
    #[test]
    fn test_computing_normal_on_transformed_sphere() {
        let m = scaling(1.0, 0.5, 1.0) * rotation_z(PI / 5.0);
        let s = Sphere::with_transform(m);
        let val = SQRT_2 / 2.0;
        let n = s.normal_at(&point(0.0, val, -val));
        check_tuple(n, vector(0.0, 0.97014, -0.24254));
    }
}
