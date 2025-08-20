use std::fmt::Debug;

use crate::{
    floats::Float,
    planes::Plane,
    shapes::{ShapeFunctions, TestShape},
    spheres::Sphere,
};

pub trait Shape: ShapeFunctions + Debug {}

impl Shape for Sphere {}
impl Shape for Plane {}
impl Shape for TestShape {}

#[derive(Copy, Clone, Debug)]
pub struct Intersection<'a> {
    pub t: Float,
    pub object: &'a dyn Shape,
}

impl<'a> Intersection<'a> {
    pub fn new(t: Float, object: &'a dyn Shape) -> Self {
        Self { t, object }
    }
}

pub fn hit<'a>(intersections: &[Intersection<'a>]) -> Option<Intersection<'a>> {
    intersections
        .iter()
        .filter(|i| i.t >= 0.0)
        .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
        .copied()
}

#[cfg(test)]
mod tests {

    use crate::{assert_same_object, floats::SQRT_2};
    use std::vec;

    use super::*;
    use crate::{
        rays::ray,
        shapes::Intersectable,
        spheres::Sphere,
        tuples::{point, vector},
    };

    // Scenario: An intersection encapsulates t and object
    //   Given s ← sphere()
    //   When i ← intersection(3.5, s)
    //   Then i.t = 3.5
    //     And i.object = s
    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, &s);
        assert_eq!(i.t, 3.5);
        assert_same_object!(i.object, &s);
    }

    // Scenario: Aggregating intersections
    //   Given s ← sphere()
    //     And i1 ← intersection(1, s)
    //     And i2 ← intersection(2, s)
    //   When xs ← intersections(i1, i2)
    //   Then xs.count = 2
    //     And xs[0].t = 1
    //     And xs[1].t = 2
    #[test]
    fn aggregating_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = [i1, i2];
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, 2.0);
    }

    // Scenario: Intersect sets the object on the intersection
    //   Given r ← ray(point(0, 0, -5), vector(0, 0, 1))
    //     And s ← sphere()
    //   When xs ← intersect(s, r)
    //   Then xs.count = 2
    //     And xs[0].object = s
    //     And xs[1].object = s
    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_same_object!(xs[0].object, &s);
        assert_same_object!(xs[1].object, &s);
    }

    // Scenario: The hit, when all intersections have positive t
    //   Given s ← sphere()
    //     And i1 ← intersection(1, s)
    //     And i2 ← intersection(2, s)
    //     And xs ← intersections(i2, i1)
    //   When i ← hit(xs)
    //   Then i = i1
    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = vec![i2, i1];
        let i = hit(&xs).unwrap();
        assert_eq!(i.t, i1.t);
        assert_same_object!(i.object, i1.object);
    }

    // Scenario: The hit, when some intersections have negative t
    //   Given s ← sphere()
    //     And i1 ← intersection(-1, s)
    //     And i2 ← intersection(1, s)
    //     And xs ← intersections(i2, i1)
    //   When i ← hit(xs)
    //   Then i = i2
    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = vec![i2, i1];
        let i = hit(&xs).unwrap();
        assert_eq!(i.t, i2.t);
        assert_same_object!(i.object, i2.object);
    }

    // Scenario: The hit, when all intersections have negative t
    //   Given s ← sphere()
    //     And i1 ← intersection(-2, s)
    //     And i2 ← intersection(-1, s)
    //     And xs ← intersections(i2, i1)
    //   When i ← hit(xs)
    //   Then i is nothing
    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = vec![i2, i1];
        let i = hit(&xs);
        assert!(i.is_none());
    }

    // Scenario: The hit is always the lowest nonnegative intersection
    //   Given s ← sphere()
    //   And i1 ← intersection(5, s)
    //   And i2 ← intersection(7, s)
    //   And i3 ← intersection(-3, s)
    //   And i4 ← intersection(2, s)
    //   And xs ← intersections(i1, i2, i3, i4)
    // When i ← hit(xs)
    // Then i = i4
    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let xs = vec![i1, i2, i3, i4];
        let i = hit(&xs).unwrap();
        assert_eq!(i.t, 2.0);
        assert_same_object!(i.object, &s);
    }

    // Scenario: Precomputing the state of an intersection
    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = Intersection::new(4.0, &shape);
        let comps = i.prepare_computations(r, None);
        assert_eq!(comps.t, i.t);
        assert_same_object!(comps.object, i.object);
        assert_eq!(comps.point, point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
    }

    // Scenario: The hit, when an intersection occurs on the outside
    //   Given r ← ray(point(0, 0, -5), vector(0, 0, 1))
    //     And shape ← sphere()
    //     And i ← intersection(4, shape)
    //   When comps ← prepare_computations(i, r)
    //   Then comps.inside = false
    #[test]
    fn when_the_hit_occurs_on_the_outside() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = Intersection::new(4.0, &shape);
        let comps = i.prepare_computations(r, None);
        assert!(!comps.inside);
    }

    // Scenario: The hit, when an intersection occurs on the inside
    //   Given r ← ray(point(0, 0, 0), vector(0, 0, 1))
    //     And shape ← sphere()
    //     And i ← intersection(1, shape)
    //   When comps ← prepare_computations(i, r)
    //   Then comps.point = point(0, 0, 1)
    //     And comps.eyev = vector(0, 0, -1)
    //     And comps.inside = true
    //       # normal would have been (0, 0, 1), but is inverted!
    //     And comps.normalv = vector(0, 0, -1)
    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = Intersection::new(1.0, &shape);
        let comps = i.prepare_computations(r, None);
        assert_eq!(comps.point, point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert!(comps.inside);
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
    }

    // Scenario: Precomputing the reflection vector
    //   Given shape ← plane()
    //     And r ← ray(point(0, 1, -1), vector(0, -√2/2, √2/2))
    //     And i ← intersection(√2, shape)
    //   When comps ← prepare_computations(i, r)
    //   Then comps.reflectv = vector(0, √2/2, √2/2)
    #[test]
    fn precomputing_the_reflection_vector() {
        let shape = Plane::new();
        let r = ray(
            point(0.0, 1.0, -1.0),
            vector(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
        );
        let i = Intersection::new(SQRT_2, &shape);
        let comps = i.prepare_computations(r, None);
        assert_eq!(comps.reflectv, vector(0.0, SQRT_2 / 2.0, SQRT_2 / 2.0));
    }

    // Scenario Outline: Finding n1 and n2 at various intersections
    //   Given A ← glass_sphere() with:
    //       | transform                 | scaling(2, 2, 2) |
    //       | material.refractive_index | 1.5              |
    //     And B ← glass_sphere() with:
    //       | transform                 | translation(0, 0, -0.25) |
    //       | material.refractive_index | 2.0                      |
    //     And C ← glass_sphere() with:
    //       | transform                 | translation(0, 0, 0.25) |
    //       | material.refractive_index | 2.5                     |
    //     And r ← ray(point(0, 0, -4), vector(0, 0, 1))
    //     And xs ← intersections(2:A, 2.75:B, 3.25:C, 4.75:B, 5.25:C, 6:A)
    //   When comps ← prepare_computations(xs[<index>], r, xs)
    //   Then comps.n1 = <n1>
    //     And comps.n2 = <n2>

    //   Examples:
    //     | index | n1  | n2  |
    //     | 0     | 1.0 | 1.5 |
    //     | 1     | 1.5 | 2.0 |
    //     | 2     | 2.0 | 2.5 |
    //     | 3     | 2.5 | 2.5 |
    //     | 4     | 2.5 | 1.5 |
    //     | 5     | 1.5 | 1.0 |
    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let mut a = crate::spheres::glass_sphere();
        a.transform = crate::transformations::scaling(2.0, 2.0, 2.0);
        a.material.refractive_index = 1.5;

        let mut b = crate::spheres::glass_sphere();
        b.transform = crate::transformations::translation(0.0, 0.0, -0.25);
        b.material.refractive_index = 2.0;

        let mut c = crate::spheres::glass_sphere();
        c.transform = crate::transformations::translation(0.0, 0.0, 0.25);
        c.material.refractive_index = 2.5;

        let r = ray(point(0.0, 0.0, -4.0), vector(0.0, 0.0, 1.0));

        let xs = vec![
            Intersection::new(2.0, &a),
            Intersection::new(2.75, &b),
            Intersection::new(3.25, &c),
            Intersection::new(4.75, &b),
            Intersection::new(5.25, &c),
            Intersection::new(6.0, &a),
        ];

        let test_cases = vec![
            (0, 1.0, 1.5),
            (1, 1.5, 2.0),
            (2, 2.0, 2.5),
            (3, 2.5, 2.5),
            (4, 2.5, 1.5),
            (5, 1.5, 1.0),
        ];

        let sxs = Some(xs.clone());
        for (index, expected_n1, expected_n2) in test_cases {
            let i = &xs[index];
            let comps = i.prepare_computations(r, sxs.clone());
            assert_eq!(comps.n1, expected_n1, "at {index}");
            assert_eq!(comps.n2, expected_n2, "at {index}");
        }
    }

    // Scenario: The under point is offset below the surface
    //   Given r ← ray(point(0, 0, -5), vector(0, 0, 1))
    //     And shape ← glass_sphere() with:
    //       | transform | translation(0, 0, 1) |
    //     And i ← intersection(5, shape)
    //     And xs ← intersections(i)
    //   When comps ← prepare_computations(i, r, xs)
    //   Then comps.under_point.z > EPSILON/2
    //     And comps.point.z < comps.under_point.z
    #[test]
    fn the_under_point_is_offset_below_the_surface() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut shape = crate::spheres::glass_sphere();
        shape.transform = crate::transformations::translation(0.0, 0.0, 1.0);
        let i = Intersection::new(5.0, &shape);
        let xs = vec![i];
        let comps = i.prepare_computations(r, Some(xs));
        assert!(comps.under_point.z > crate::floats::EPSILON / 2.0);
        assert!(comps.point.z < comps.under_point.z);
    }
}
