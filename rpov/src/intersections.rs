use std::fmt::Debug;

use crate::{floats::Float, spheres::Sphere};
// You can still derive Copy and Clone, as the reference is copyable.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Intersection<'a> {
    pub t: Float,
    pub object: &'a Sphere,
}

impl<'a> Intersection<'a> {
    pub fn new(t: Float, object: &'a Sphere) -> Self {
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
    use std::vec;

    use super::*;
    use crate::{
        rays::ray, shapes::ShapeFunctions, spheres::Sphere, tuples::point, tuples::vector,
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
        assert_eq!(i.object, &s);
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
        let xs = vec![i1, i2];
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
        assert_eq!(xs[0].object, &s);
        assert_eq!(xs[1].object, &s);
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
        let i = hit(&xs);
        assert_eq!(i, Some(i1));
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
        let i = hit(&xs);
        assert_eq!(i, Some(i2));
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
        assert_eq!(i, None);
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
        let i = hit(&xs);
        assert_eq!(i, Some(i4));
        assert_eq!(i.unwrap().t, 2.0);
        assert_eq!(i.unwrap().object, &s);
    }

    // Scenario: Precomputing the state of an intersection
    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = Intersection::new(4.0, &shape);
        let comps = i.prepare_computations(r);
        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
    }

    // Scenario: When the hit occurs on the outside
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
        let comps = i.prepare_computations(r);
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
        let comps = i.prepare_computations(r);
        assert_eq!(comps.point, point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert!(comps.inside);
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
    }
}
