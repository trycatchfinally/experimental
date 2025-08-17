use crate::{
    intersections::Intersection, materials::Material, matrices::Matrix4, rays::Ray,
    shapes::ShapeFunctions, tuples::Tuple4, tuples::vector,
};

struct Plane {
    transform: Matrix4,
    material: Material,
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            transform: Matrix4::identity(),
            material: Material::new(),
        }
    }
}

impl ShapeFunctions for Plane {
    fn transform_inverse(&self) -> Matrix4 {
        self.transform.inverse()
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn local_intersect<'a>(&'a self, _local_ray: Ray) -> Vec<Intersection<'a>> {
        vec![]
    }

    fn local_normal_at(&self, _local_point: &Tuple4) -> Tuple4 {
        vector(0.0, 1.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuples::{point, vector};

    // Scenario: The normal of a plane is constant everywhere
    //   Given p ← plane()
    //   When n1 ← local_normal_at(p, point(0, 0, 0))
    //     And n2 ← local_normal_at(p, point(10, 0, -10))
    //     And n3 ← local_normal_at(p, point(-5, 0, 150))
    //   Then n1 = vector(0, 1, 0)
    //     And n2 = vector(0, 1, 0)
    //     And n3 = vector(0, 1, 0)
    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = Plane::default();
        let n1 = p.local_normal_at(&point(0.0, 0.0, 0.0));
        let n2 = p.local_normal_at(&point(10.0, 0.0, -10.0));
        let n3 = p.local_normal_at(&point(-5.0, 0.0, 150.0));
        assert_eq!(n1, vector(0.0, 1.0, 0.0));
        assert_eq!(n2, vector(0.0, 1.0, 0.0));
        assert_eq!(n3, vector(0.0, 1.0, 0.0));
    }

    // Scenario: Intersect with a ray parallel to the plane
    //   Given p ← plane()
    //     And r ← ray(point(0, 10, 0), vector(0, 0, 1))
    //   When xs ← local_intersect(p, r)
    //   Then xs is empty
    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let p = Plane::default();
        let r = Ray::new(point(0.0, 10.0, 0.0), vector(0.0, 0.0, 1.0));
        let xs = p.local_intersect(r);
        assert!(xs.is_empty());
    }

    // Scenario: Intersect with a coplanar ray
    //   Given p ← plane()
    //     And r ← ray(point(0, 0, 0), vector(0, 0, 1))
    //   When xs ← local_intersect(p, r)
    //   Then xs is empty
    #[test]
    fn intersect_with_a_coplanar_ray() {
        let p = Plane::default();
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let xs = p.local_intersect(r);
        assert!(xs.is_empty());
    }

    // Scenario: A ray intersecting a plane from above
    //   Given p ← plane()
    //     And r ← ray(point(0, 1, 0), vector(0, -1, 0))
    //   When xs ← local_intersect(p, r)
    //   Then xs.count = 1
    //     And xs[0].t = 1
    //     And xs[0].object = p
    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let p = Plane::default();
        let r = Ray::new(point(0.0, 1.0, 0.0), vector(0.0, -1.0, 0.0));
        let xs = p.local_intersect(r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[0].object, &p);
    }

    // Scenario: A ray intersecting a plane from below
    //   Given p ← plane()
    //     And r ← ray(point(0, -1, 0), vector(0, 1, 0))
    //   When xs ← local_intersect(p, r)
    //   Then xs.count = 1
    //     And xs[0].t = 1
    //     And xs[0].object = p
    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p = Plane::default();
        let r = Ray::new(point(0.0, -1.0, 0.0), vector(0.0, 1.0, 0.0));
        let xs = p.local_intersect(r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[0].object, &p);
    }
}
