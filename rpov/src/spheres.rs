use crate::intersections::Intersection;
use crate::materials::Material;
use crate::matrices::Matrix4;
use crate::rays::Ray;
use crate::shapes::Intersectable;
use crate::shapes::ShapeFunctions;
use crate::tuples::Tuple4;
use crate::tuples::point;
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

static SPHERE_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone)]
pub struct Sphere {
    pub id: u64,
    pub transform: Matrix4,
    pub material: Material,
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl fmt::Display for Sphere {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Sphere(id={}, transform={:?})", self.id, self.transform)
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new()
    }
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            id: SPHERE_ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            transform: Matrix4::identity(),
            material: Material::new(),
        }
    }

    pub fn with_transform(transform: Matrix4) -> Self {
        Self {
            id: SPHERE_ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            transform,
            material: Material::new(),
        }
    }
}
impl ShapeFunctions for Sphere {
    fn transform_inverse(&self) -> Matrix4 {
        self.transform.inverse()
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn local_normal_at(&self, local_point: &Tuple4) -> Tuple4 {
        *local_point - point(0.0, 0.0, 0.0)
    }
}

impl Intersectable<Sphere> for Sphere {
    fn local_intersect<'a>(&'a self, local_ray: Ray) -> Vec<Intersection<'a>> {
        // let local_ray = r.transform(self.transform.inverse());
        let sphere_to_ray = local_ray.origin - point(0.0, 0.0, 0.0);

        let a = local_ray.direction.dot(local_ray.direction);
        let b = 2.0 * local_ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec![];
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        if t1 > t2 {
            return vec![Intersection::new(t2, self), Intersection::new(t1, self)];
        }

        vec![Intersection::new(t1, self), Intersection::new(t2, self)]
    }
}

pub fn glass_sphere() -> Sphere {
    let mut s = Sphere::new();
    s.material.transparency = 1.0;
    s.material.refractive_index = 1.5;
    s
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::check_floats;
    use crate::floats::SQRT_2;
    use crate::rays::ray;
    use crate::transformations::{scaling, translation};
    use crate::tuples::vector;

    // Scenario: A ray intersects a sphere at two points
    //   Given r ← ray(point(0, 0, -5), vector(0, 0, 1))
    //     And s ← sphere()
    //   When xs ← intersect(s, r)
    //   Then xs.count = 2
    //     And xs[0] = 4.0
    //     And xs[1] = 6.0
    #[test]
    fn test_ray_intersects_sphere_at_two_points() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    // Scenario: A ray intersects a sphere at a tangent
    //   Given r ← ray(point(0, 1, -5), vector(0, 0, 1))
    //     And s ← sphere()
    //   When xs ← intersect(s, r)
    //   Then xs.count = 2
    //     And xs[0] = 5.0
    //     And xs[1] = 5.0
    #[test]
    fn test_ray_intersects_sphere_at_a_tangent() {
        let r = ray(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    // Scenario: A ray misses a sphere
    //   Given r ← ray(point(0, 2, -5), vector(0, 0, 1))
    //     And s ← sphere()
    //   When xs ← intersect(s, r)
    //   Then xs.count = 0
    #[test]
    fn test_ray_misses_a_sphere() {
        let r = ray(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 0);
    }

    // Scenario: A ray originates inside a sphere
    //   Given r ← ray(point(0, 0, 0), vector(0, 0, 1))
    //     And s ← sphere()
    //   When xs ← intersect(s, r)
    //   Then xs.count = 2
    //     And xs[0] = -1.0
    //     And xs[1] = 1.0
    #[test]
    fn test_ray_originates_inside_a_sphere() {
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    // Scenario: A sphere is behind a ray
    //   Given r ← ray(point(0, 0, 5), vector(0, 0, 1))
    //     And s ← sphere()
    //   When xs ← intersect(s, r)
    //   Then xs.count = 2
    //     And xs[0] = -6.0
    //     And xs[1] = -4.0
    #[test]
    fn test_sphere_is_behind_a_ray() {
        let r = ray(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn two_spheres_have_different_ids() {
        let s1 = Sphere::new();
        let s2 = Sphere::new();
        assert_ne!(s1.id, s2.id);
    }

    // Scenario: A sphere's default transformation
    //   Given s ← sphere()
    //   Then s.transform = identity_matrix
    #[test]
    fn a_sphere_default_transformation() {
        let s = Sphere::new();
        assert_eq!(s.transform, Matrix4::identity());
    }

    // Scenario: Changing a sphere's transformation
    //   Given s ← sphere()
    //     And t ← translation(2, 3, 4)
    //   When set_transform(s, t)
    //   Then s.transform = t
    #[test]
    fn changing_a_sphere_transformation() {
        let t = translation(2.0, 3.0, 4.0);
        let s = Sphere::with_transform(t);
        assert_eq!(s.transform, t);
    }

    // Scenario: Intersecting a scaled sphere with a ray
    //   Given r ← ray(point(0, 0, -5), vector(0, 0, 1))
    //     And s ← sphere()
    //   When set_transform(s, scaling(2, 2, 2))
    //     And xs ← intersect(s, r)
    //   Then xs.count = 2
    //     And xs[0].t = 3
    //     And xs[1].t = 7
    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::with_transform(scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    // Scenario: Intersecting a translated sphere with a ray
    //   Given r ← ray(point(0, 0, -5), vector(0, 0, 1))
    //     And s ← sphere()
    //   When set_transform(s, translation(5, 0, 0))
    //     And xs ← intersect(s, r)
    //   Then xs.count = 0
    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = Sphere::with_transform(translation(5.0, 0.0, 0.0));
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 0);
    }

    //   Scenario: A helper for producing a sphere with a glassy material
    //   Given s ← glass_sphere()
    //   Then s.transform = identity_matrix
    //     And s.material.transparency = 1.0
    //     And s.material.refractive_index = 1.5
    #[test]
    fn a_helper_for_producing_a_sphere_with_a_glassy_material() {
        let s = glass_sphere();
        assert_eq!(s.transform, Matrix4::identity());
        assert_eq!(s.material.transparency, 1.0);
        assert_eq!(s.material.refractive_index, 1.5);
    }

    // Scenario: The Schlick approximation under total internal reflection
    //   Given shape ← glass_sphere()
    //     And r ← ray(point(0, 0, √2/2), vector(0, 1, 0))
    //     And xs ← intersections(-√2/2:shape, √2/2:shape)
    //   When comps ← prepare_computations(xs[1], r, xs)
    //     And reflectance ← schlick(comps)
    //   Then reflectance = 1.0
    #[test]
    fn the_schlick_approximation_under_total_internal_reflection() {
        let shape = glass_sphere();
        let r = ray(point(0.0, 0.0, SQRT_2 / 2.0), vector(0.0, 1.0, 0.0));
        let xs = vec![
            Intersection::new(-SQRT_2 / 2.0, &shape),
            Intersection::new(SQRT_2 / 2.0, &shape),
        ];
        let comps = xs[1].prepare_computations(r, Some(xs.clone()));
        let reflectance = crate::lighting::schlick(&comps);
        assert_eq!(reflectance, 1.0);
    }

    // Scenario: The Schlick approximation with a perpendicular viewing angle
    //   Given shape ← glass_sphere()
    //     And r ← ray(point(0, 0, 0), vector(0, 1, 0))
    //     And xs ← intersections(-1:shape, 1:shape)
    //   When comps ← prepare_computations(xs[1], r, xs)
    //     And reflectance ← schlick(comps)
    //   Then reflectance = 0.04
    #[test]
    fn the_schlick_approximation_with_a_perpendicular_viewing_angle() {
        let shape = glass_sphere();
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 1.0, 0.0));
        let xs = vec![
            Intersection::new(-1.0, &shape),
            Intersection::new(1.0, &shape),
        ];
        let comps = xs[1].prepare_computations(r, Some(xs.clone()));
        let reflectance = crate::lighting::schlick(&comps);
        check_floats!(reflectance, 0.04);
    }

    // Scenario: The Schlick approximation with small angle and n2 > n1
    //   Given shape ← glass_sphere()
    //     And r ← ray(point(0, 0.99, -2), vector(0, 0, 1))
    //     And xs ← intersections(1.8589:shape)
    //   When comps ← prepare_computations(xs[0], r, xs)
    //     And reflectance ← schlick(comps)
    //   Then reflectance = 0.48873
    #[test]
    fn the_schlick_approximation_with_small_angle_and_n2_gt_n1() {
        let shape = glass_sphere();
        let r = ray(point(0.0, 0.99, -2.0), vector(0.0, 0.0, 1.0));
        let xs = vec![Intersection::new(1.8589, &shape)];
        let comps = xs[0].prepare_computations(r, Some(xs.clone()));
        let reflectance = crate::lighting::schlick(&comps);
        assert!((reflectance - 0.48873).abs() < crate::floats::EPSILON);
    }
}
