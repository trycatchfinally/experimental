
#[cfg(test)]
mod tests {
    use super::*;

    // Scenario: A tuple with w=1.0 is a point
    //   Given a ← tuple(4.3, -4.2, 3.1, 1.0)
    //   Then a.x = 4.3
    //     And a.y = -4.2
    //     And a.z = 3.1
    //     And a.w = 1.0
    //     And a is a point
    //     And a is not a vector
    #[test]
    fn a_tuple_with_w_1_is_a_point() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    // Scenario: A tuple with w=0 is a vector
    //   Given a ← tuple(4.3, -4.2, 3.1, 0.0)
    //   Then a.x = 4.3
    //     And a.y = -4.2
    //     And a.z = 3.1
    //     And a.w = 0.0
    //     And a is not a point
    //     And a is a vector
    #[test]
    fn a_tuple_with_w_0_is_a_vector() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.0);
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    // Scenario: point() creates tuples with w=1
    //   Given p ← point(4, -4, 3)
    //   Then p = tuple(4, -4, 3, 1)
    #[test]
    fn point_creates_tuples_with_w_1() {
        let p = point(4.0, -4.0, 3.0);
        assert_eq!(p, Tuple::new(4.0, -4.0, 3.0, 1.0));
    }

    // Scenario: vector() creates tuples with w=0
    //   Given v ← vector(4, -4, 3)
    //   Then v = tuple(4, -4, 3, 0)
    #[test]
    fn vector_creates_tuples_with_w_0() {
        let v = vector(4.0, -4.0, 3.0);
        assert_eq!(v, Tuple::new(4.0, -4.0, 3.0, 0.0));
    }

    // Scenario: Adding two tuples
    //   Given a1 ← tuple(3, -2, 5, 1)
    //     And a2 ← tuple(-2, 3, 1, 0)
    //    Then a1 + a2 = tuple(1, 1, 6, 1)
    #[test]
    fn adding_two_tuples() {
        let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);
        assert_eq!(a1 + a2, Tuple::new(1.0, 1.0, 6.0, 1.0));
    }

    // Scenario: Subtracting two points
    //   Given p1 ← point(3, 2, 1)
    //     And p2 ← point(5, 6, 7)
    //   Then p1 - p2 = vector(-2, -4, -6)
    #[test]
    fn subtracting_two_points() {
        let p1 = point(3.0, 2.0, 1.0);
        let p2 = point(5.0, 6.0, 7.0);
        assert_eq!(p1 - p2, vector(-2.0, -4.0, -6.0));
    }

    // Scenario: Subtracting a vector from a point
    //   Given p ← point(3, 2, 1)
    //     And v ← vector(5, 6, 7)
    //   Then p - v = point(-2, -4, -6)
    #[test]
    fn subtracting_a_vector_from_a_point() {
        let p = point(3.0, 2.0, 1.0);
        let v = vector(5.0, 6.0, 7.0);
        assert_eq!(p - v, point(-2.0, -4.0, -6.0));
    }

    // Scenario: Subtracting two vectors
    //   Given v1 ← vector(3, 2, 1)
    //     And v2 ← vector(5, 6, 7)
    //   Then v1 - v2 = vector(-2, -4, -6)
    #[test]
    fn subtracting_two_vectors() {
        let v1 = vector(3.0, 2.0, 1.0);
        let v2 = vector(5.0, 6.0, 7.0);
        assert_eq!(v1 - v2, vector(-2.0, -4.0, -6.0));
    }

    // Scenario: Subtracting a vector from the zero vector
    //   Given zero ← vector(0, 0, 0)
    //     And v ← vector(1, -2, 3)
    //   Then zero - v = vector(-1, 2, -3)
    #[test]
    fn subtracting_a_vector_from_the_zero_vector() {
        let zero = vector(0.0, 0.0, 0.0);
        let v = vector(1.0, -2.0, 3.0);
        assert_eq!(zero - v, vector(-1.0, 2.0, -3.0));
    }

    // Scenario: Negating a tuple
    //   Given a ← tuple(1, -2, 3, -4)
    //   Then -a = tuple(-1, 2, -3, 4)
    #[test]
    fn negating_a_tuple() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(-a, Tuple::new(-1.0, 2.0, -3.0, 4.0));
    }

    // Scenario: Multiplying a tuple by a scalar
    //   Given a ← tuple(1, -2, 3, -4)
    //   Then a * 3.5 = tuple(3.5, -7, 10.5, -14)
    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 3.5, Tuple::new(3.5, -7.0, 10.5, -14.0));
    }

    // Scenario: Multiplying a tuple by a fraction
    //   Given a ← tuple(1, -2, 3, -4)
    //   Then a * 0.5 = tuple(0.5, -1, 1.5, -2)
    #[test]
    fn multiplying_a_tuple_by_a_fraction() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 0.5, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    // Scenario: Multiplying a negated tuple by a fraction
    //   Given a ← tuple(1, -2, 3, -4)
    //   Then -a * 0.5 = tuple(-0.5, 1, -1.5, 2)
    #[test]
    fn multiplying_a_negated_tuple_by_a_fraction() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(-a * 0.5, Tuple::new(-0.5, 1.0, -1.5, 2.0));
    }

    // Scenario: Dividing a tuple by a scalar
    //   Given a ← tuple(1, -2, 3, -4)
    //   Then a / 2 = tuple(0.5, -1, 1.5, -2)
    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a / 2.0, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    // Scenario: Computing the magnitude of vector(1, 0, 0)
    //   Given v ← vector(1, 0, 0)
    //   Then magnitude(v) = 1
    #[test]
    fn computing_the_magnitude_of_vector_1_0_0() {
        let v = vector(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    // Scenario: Computing the magnitude of vector(0, 1, 0)
    //   Given v ← vector(0, 1, 0)
    //   Then magnitude(v) = 1
    #[test]
    fn computing_the_magnitude_of_vector_0_1_0() {
        let v = vector(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    // Scenario: Computing the magnitude of vector(0, 0, 1)
    //   Given v ← vector(0, 0, 1)
    //   Then magnitude(v) = 1
    #[test]
    fn computing_the_magnitude_of_vector_0_0_1() {
        let v = vector(0.0, 0.0, 1.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    // Scenario: Computing the magnitude of vector(1, 2, 3)
    //   Given v ← vector(1, 2, 3)
    //   Then magnitude(v) = √14
    #[test]
    fn computing_the_magnitude_of_vector_1_2_3() {
        let v = vector(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), (14.0_f64).sqrt());
    }

    // Scenario: Computing the magnitude of vector(-1, -2, -3)
    //   Given v ← vector(-1, -2, -3)
    //   Then magnitude(v) = √14
    #[test]
    fn computing_the_magnitude_of_vector_neg_1_neg_2_neg_3() {
        let v = vector(-1.0, -2.0, -3.0);
        assert_eq!(v.magnitude(), (14.0_f64).sqrt());
    }

    // Scenario: Normalizing vector(4, 0, 0) gives (1, 0, 0)
    //   Given v ← vector(4, 0, 0)
    //   Then normalize(v) = vector(1, 0, 0)
    #[test]
    fn normalizing_vector_4_0_0_gives_1_0_0() {
        let v = vector(4.0, 0.0, 0.0);
        assert_eq!(v.normalize(), vector(1.0, 0.0, 0.0));
    }

    // Scenario: Normalizing vector(1, 2, 3)
    //   Given v ← vector(1, 2, 3)
    //                                   # vector(1/√14,   2/√14,   3/√14)
    //   Then normalize(v) = approximately vector(0.26726, 0.53452, 0.80178)
    #[test]
    fn normalizing_vector_1_2_3() {
        let v = vector(1.0, 2.0, 3.0);
        let sqrt14 = (14.0_f64).sqrt();
        assert_eq!(
            v.normalize(),
            vector(1.0 / sqrt14, 2.0 / sqrt14, 3.0 / sqrt14)
        );
    }

    // Scenario: The magnitude of a normalized vector
    //   Given v ← vector(1, 2, 3)
    //   When norm ← normalize(v)
    //   Then magnitude(norm) = 1
    #[test]
    fn the_magnitude_of_a_normalized_vector() {
        let v = vector(1.0, 2.0, 3.0);
        let norm = v.normalize();
        assert_eq!(norm.magnitude(), 1.0);
    }

    // Scenario: The dot product of two tuples
    //   Given a ← vector(1, 2, 3)
    //     And b ← vector(2, 3, 4)
    //   Then dot(a, b) = 20
    #[test]
    fn the_dot_product_of_two_tuples() {
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
        assert_eq!(dot(&a, &b), 20.0);
    }

    // Scenario: The cross product of two vectors
    //   Given a ← vector(1, 2, 3)
    //     And b ← vector(2, 3, 4)
    //   Then cross(a, b) = vector(-1, 2, -1)
    //     And cross(b, a) = vector(1, -2, 1)
    #[test]
    fn the_cross_product_of_two_vectors() {
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
        assert_eq!(cross(&a, &b), vector(-1.0, 2.0, -1.0));
        assert_eq!(cross(&b, &a), vector(1.0, -2.0, 1.0));
    }

    // Scenario: Reflecting a vector approaching at 45°
    //   Given v ← vector(1, -1, 0)
    //     And n ← vector(0, 1, 0)
    //   When r ← reflect(v, n)
    //   Then value(r) = vector(1, 1, 0)
    #[test]
    fn reflecting_a_vector_approaching_at_45_degrees() {
        let v = vector(1.0, -1.0, 0.0);
        let n = vector(0.0, 1.0, 0.0);
        let r = reflect(&v, &n);
        assert_eq!(r, vector(1.0, 1.0, 0.0));
    }
    // Scenario: Reflecting a vector off a slanted surface
    //   Given v ← vector(0, -1, 0)
    //     And n ← vector(√2/2, √2/2, 0)
    //   When r ← reflect(v, n)
    //   Then r = vector(1, 0, 0)
    #[test]
    fn reflecting_a_vector_off_a_slanted_surface() {
        let v = vector(0.0, -1.0, 0.0);
        let n = vector((2.0_f64).sqrt() / 2.0, (2.0_f64).sqrt() / 2.0, 0.0);
        let r = reflect(&v, &n);
        assert_eq!(r, vector(1.0, 0.0, 0.0));
    }
}
