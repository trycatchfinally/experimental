use std::cell::RefCell;

use crate::intersections::Intersection;
use crate::materials::Material;
use crate::matrices::Matrix4;
use crate::rays::Ray;
use crate::tuples::{Tuple4, point};

#[derive(Clone, Debug, PartialEq)]
pub struct TestShape {
    pub transform: Matrix4,
    pub material: Material,
    pub saved_ray: RefCell<Option<Ray>>,
}

impl Default for TestShape {
    fn default() -> Self {
        Self::new()
    }
}

impl TestShape {
    pub fn new() -> Self {
        TestShape {
            transform: Matrix4::identity(),
            material: Material::new(),
            saved_ray: RefCell::new(None),
        }
    }
}
pub trait ShapeFunctions {
    fn intersect<'a>(&'a self, ray: Ray) -> Vec<Intersection<'a>> {
        let local_ray = ray.transform(self.transform_inverse());
        self.local_intersect(local_ray)
    }

    fn normal_at(&self, world_point: &Tuple4) -> Tuple4 {
        let ti = self.transform_inverse();
        let local_point = ti * *world_point;
        let local_normal = self.local_normal_at(&local_point);
        let mut world_normal = ti.transpose() * local_normal;
        world_normal.w = 0.0;
        world_normal.normalize()
    }

    fn transform_inverse(&self) -> Matrix4;
    fn local_intersect<'a>(&'a self, local_ray: Ray) -> Vec<Intersection<'a>>;
    fn local_normal_at(&self, local_point: &Tuple4) -> Tuple4;
}

impl ShapeFunctions for TestShape {
    fn transform_inverse(&self) -> Matrix4 {
        self.transform.inverse()
    }

    fn local_intersect<'a>(&'a self, local_ray: Ray) -> Vec<Intersection<'a>> {
        *self.saved_ray.borrow_mut() = Some(local_ray);
        vec![]
    }

    fn local_normal_at(&self, world_point: &Tuple4) -> Tuple4 {
        point(world_point.x, world_point.y, world_point.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::floats::{FRAC_1_SQRT_2, PI, SQRT_2};
    use crate::materials::Material;
    use crate::matrices::{self, Matrix4};
    use crate::rays::ray;
    use crate::transformations::{rotation_z, scaling, translation};
    use crate::tuples::{check_tuple, point, vector};

    fn test_shape() -> TestShape {
        TestShape::new()
    }

    fn set_transform(shape: &mut TestShape, transform: Matrix4) {
        shape.transform = transform;
    }

    // Scenario: The default transformation
    //   Given s ← test_shape()
    //   Then s.transform = identity_matrix
    #[test]
    fn the_default_transformation() {
        let s = test_shape();
        assert_eq!(s.transform, Matrix4::identity());
    }

    // Scenario: Assigning a transformation
    //   Given s ← test_shape()
    //   When set_transform(s, translation(2, 3, 4))
    //   Then s.transform = translation(2, 3, 4)
    #[test]
    fn assigning_a_transformation() {
        let mut s = test_shape();
        set_transform(&mut s, translation(2.0, 3.0, 4.0));
        matrices::check(s.transform, translation(2.0, 3.0, 4.0));
    }

    // Scenario: The default material
    //   Given s ← test_shape()
    //   When m ← s.material
    //   Then m = material()
    #[test]
    fn the_default_material() {
        let s = test_shape();
        let m = s.material;
        assert_eq!(m, Material::new());
    }

    // Scenario: Assigning a material
    //   Given s ← test_shape()
    //     And m ← material()
    //     And m.ambient ← 1
    //   When s.material ← m
    //   Then s.material = m
    #[test]
    fn assigning_a_material() {
        let mut s = test_shape();
        let mut m = Material::new();
        m.ambient = 1.0;
        s.material = m;
        assert_eq!(s.material, m);
    }

    // Scenario: Intersecting a scaled shape with a ray
    //   Given r ← ray(point(0, 0, -5), vector(0, 0, 1))
    //     And s ← test_shape()
    //   When set_transform(s, scaling(2, 2, 2))
    //     And xs ← intersect(s, r)
    //   Then s.saved_ray.origin = point(0, 0, -2.5)
    //     And s.saved_ray.direction = vector(0, 0, 0.5)
    #[test]
    fn intersecting_a_scaled_shape_with_a_ray() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = test_shape();
        set_transform(&mut s, scaling(2.0, 2.0, 2.0));
        s.intersect(r);
        let saved_ray = s.saved_ray.borrow().unwrap();
        check_tuple(saved_ray.origin, point(0.0, 0.0, -2.5));
        check_tuple(saved_ray.direction, vector(0.0, 0.0, 0.5));
    }

    // Scenario: Intersecting a translated shape with a ray
    //   Given r ← ray(point(0, 0, -5), vector(0, 0, 1))
    //     And s ← test_shape()
    //   When set_transform(s, translation(5, 0, 0))
    //     And xs ← intersect(s, r)
    //   Then s.saved_ray.origin = point(-5, 0, -5)
    //     And s.saved_ray.direction = vector(0, 0, 1)
    #[test]
    fn intersecting_a_translated_shape_with_a_ray() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = test_shape();
        set_transform(&mut s, translation(5.0, 0.0, 0.0));
        s.intersect(r);
        let saved_ray = s.saved_ray.borrow().unwrap();
        assert_eq!(saved_ray.origin, point(-5.0, 0.0, -5.0));
        assert_eq!(saved_ray.direction, vector(0.0, 0.0, 1.0));
    }

    // Scenario: Computing the normal on a translated shape
    //   Given s ← test_shape()
    //   When set_transform(s, translation(0, 1, 0))
    //     And n ← normal_at(s, point(0, 1.70711, -floats::FRAC_1_SQRT_2))
    //   Then n = vector(0, floats::FRAC_1_SQRT_2, -floats::FRAC_1_SQRT_2)
    #[test]
    fn computing_the_normal_on_a_translated_shape() {
        let mut s = test_shape();
        set_transform(&mut s, translation(0.0, 1.0, 0.0));
        let n = s.normal_at(&point(0.0, 1.70711, -FRAC_1_SQRT_2));
        check_tuple(n, vector(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
    }

    // Scenario: Computing the normal on a transformed shape
    //   Given s ← test_shape()
    //     And m ← scaling(1, 0.5, 1) * rotation_z(π/5)
    //   When set_transform(s, m)
    //     And n ← normal_at(s, point(0, √2/2, -√2/2))
    //   Then n = vector(0, 0.97014, -0.24254)
    #[test]
    fn computing_the_normal_on_a_transformed_shape() {
        let mut s = test_shape();
        let m = scaling(1.0, 0.5, 1.0) * rotation_z(PI / 5.0);
        set_transform(&mut s, m);
        let n = s.normal_at(&point(0.0, SQRT_2 / 2.0, -SQRT_2 / 2.0));
        check_tuple(n, vector(0.0, 0.97014, -0.24254));
    }
}
