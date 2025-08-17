use std::ops::Deref;

use indicatif::{ProgressBar, ProgressStyle};

use crate::{
    canvas::Canvas,
    colors::Color,
    floats::{EPSILON, Float},
    intersections::{Intersection, Shape, hit},
    lighting::{PointLight, point_light},
    materials::Material,
    rays::Ray,
    shapes::Intersectable,
    spheres::Sphere,
    transformations::scaling,
    tuples::{Tuple4, point},
};

pub struct World {
    pub objects: Vec<Sphere>,
    pub light: Option<PointLight>,
}

pub struct Computations<'a> {
    pub t: Float,
    pub object: &'a dyn Shape,
    pub point: Tuple4,
    pub eyev: Tuple4,
    pub normalv: Tuple4,
    pub inside: bool,
    pub over_point: Tuple4,
    pub reflectv: Tuple4,
}

pub type Intersections<'a> = Vec<Intersection<'a>>;

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            light: None,
        }
    }

    pub fn intersect(&self, r: Ray) -> Intersections {
        let mut all_intersections = Vec::new();
        for object in &self.objects {
            let mut intersections = object.intersect(r);
            all_intersections.append(&mut intersections);
        }
        all_intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        all_intersections
    }
    pub fn shade_hit(&self, comps: Computations) -> Color {
        let light = self.light.as_ref().expect("Light source not set in world");
        let in_shadow = self.is_shadowed(comps.over_point);
        crate::lighting::lighting(
            comps.object.material(),
            comps.object,
            light,
            comps.over_point,
            comps.eyev,
            comps.normalv,
            in_shadow,
        )
    }

    pub fn color_at(&self, r: Ray) -> Color {
        let xs = self.intersect(r);
        let hit = crate::intersections::hit(&xs);
        match hit {
            Some(i) => {
                let comps = i.prepare_computations(r);
                self.shade_hit(comps)
            }
            None => Color::new(0.0, 0.0, 0.0),
        }
    }

    pub fn is_shadowed(&self, point: Tuple4) -> bool {
        let light = self.light.as_ref().expect("Light source not set in world");
        let v = light.position - point;
        let distance = v.magnitude();
        let direction = v.normalize();

        let r = Ray::new(point, direction);
        let intersections = self.intersect(r);

        let h = hit(&intersections);
        h.is_some() && h.unwrap().t < distance
    }

    pub fn reflected_color(&self, comps: Computations) -> Color {
        let reflect_ray = Ray::new(comps.over_point, comps.reflectv);
        let color = self.color_at(reflect_ray);
        color * comps.object.material().reflective
    }
}

pub fn render(c: crate::camera::Camera, w: World) -> Canvas {
    let mut image = Canvas::new(c.hsize, c.vsize);

    let bar = ProgressBar::new(c.vsize as u64);
    bar.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta}) {msg}")
            .unwrap()
            .progress_chars("#>- "));
    bar.set_message("Rendering...".to_string());

    for y in 0..c.vsize {
        bar.inc(1);
        for x in 0..c.hsize {
            let r = c.ray_for_pixel(x, y);
            let color = w.color_at(r);
            image.write_pixel(x, y, color);
        }
    }
    bar.finish_and_clear();
    image
}

pub fn default_world() -> World {
    let light = point_light(point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let mut s1 = Sphere::new();
    s1.material = Material {
        color: Color::new(0.8, 1.0, 0.6),
        diffuse: 0.7,
        specular: 0.2,
        ..Material::new()
    };

    let s2 = Sphere::with_transform(scaling(0.5, 0.5, 0.5));

    World {
        objects: vec![s1, s2],
        light: Some(light),
    }
}

impl<'a> Intersection<'a> {
    pub fn prepare_computations(&self, ray: Ray) -> Computations {
        let point = ray.position(self.t);
        let eyev = -ray.direction;
        let mut normalv = self.object.normal_at(&point);
        let inside = normalv.dot(eyev) < 0.0;
        if inside {
            normalv = -normalv;
        }
        let reflectv = ray.direction.reflect(normalv);
        let over_point = point + normalv * EPSILON;
        Computations {
            t: self.t,
            object: self.object,
            point,
            eyev,
            normalv,
            inside,
            over_point,
            reflectv,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        floats::{PI, SQRT_2},
        planes::Plane,
        rays::ray,
        transformations::scaling,
        tuples::vector,
    };

    // Scenario: Creating a world
    //   Given w ← world()
    //   Then w contains no objects
    //     And w has no light source
    #[test]
    fn creating_a_world() {
        let w = World::new();
        assert!(w.objects.is_empty());
        assert!(w.light.is_none());
    }

    // Scenario: The default world
    //   Given light ← point_light(point(-10, 10, -10), color(1, 1, 1))
    //     And s1 ← sphere() with:
    //       | material.color     | (0.8, 1.0, 0.6)        |
    //       | material.diffuse   | 0.7                    |
    //       | material.specular  | 0.2                    |
    //     And s2 ← sphere() with:
    //       | transform | scaling(0.5, 0.5, 0.5) |
    //   When w ← default_world()
    //   Then w.light = light
    //     And w contains s1
    //     And w contains s2
    #[test]
    fn the_default_world() {
        let light = point_light(point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let mut s1 = Sphere::new();
        s1.material = Material {
            color: Color::new(0.8, 1.0, 0.6),
            diffuse: 0.7,
            specular: 0.2,
            ..Material::new()
        };

        let mut s2 = Sphere::with_transform(scaling(0.5, 0.5, 0.5));

        let w = default_world();
        assert_eq!(w.light.unwrap(), light);
        s1.id = w.objects[0].id;
        s2.id = w.objects[1].id;
        assert!(w.objects.contains(&s1));
        assert!(w.objects.contains(&s2));
    }

    // Scenario: Intersect a world with a ray
    //   Given w ← default_world()
    //     And r ← ray(point(0, 0, -5), vector(0, 0, 1))
    //   When xs ← intersect_world(w, r)
    //   Then xs.count = 4
    //     And xs[0].t = 4
    //     And xs[1].t = 4.5
    //     And xs[2].t = 5.5
    //     And xs[3].t = 6
    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = default_world();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = w.intersect(r);
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }

    // Scenario: Shading an intersection
    //   Given w ← default_world()
    //     And r ← ray(point(0, 0, -5), vector(0, 0, 1))
    //     And shape ← the first object in w
    //     And i ← intersection(4, shape)
    //   When comps ← prepare_computations(i, r)
    //     And c ← shade_hit(w, comps)
    //   Then c = color(0.38066, 0.47583, 0.2855)
    #[test]
    fn shading_an_intersection() {
        let w = default_world();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = &w.objects[0];
        let i = Intersection::new(4.0, shape);
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    // Scenario: Shading an intersection from the inside
    //   Given w ← default_world()
    //     And w.light ← point_light(point(0, 0.25, 0), color(1, 1, 1))
    //     And r ← ray(point(0, 0, 0), vector(0, 0, 1))
    //     And shape ← the second object in w
    //     And i ← intersection(0.5, shape)
    //   When comps ← prepare_computations(i, r)
    //     And c ← shade_hit(w, comps)
    //   Then c = color(0.90498, 0.90498, 0.90498)
    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = default_world();
        w.light = Some(point_light(
            point(0.0, 0.25, 0.0),
            Color::new(1.0, 1.0, 1.0),
        ));
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = &w.objects[1];
        let i = Intersection::new(0.5, shape);
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    // Scenario: The color when a ray misses
    //   Given w ← default_world()
    //     And r ← ray(point(0, 0, -5), vector(0, 1, 0))
    //   When c ← color_at(w, r)
    //   Then c = color(0, 0, 0)
    #[test]
    fn the_color_when_a_ray_misses() {
        let w = default_world();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0));
        let c = w.color_at(r);
        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }

    // Scenario: The color when a ray hits
    //   Given w ← default_world()
    //     And r ← ray(point(0, 0, -5), vector(0, 0, 1))
    //   When c ← color_at(w, r)
    //   Then c = color(0.38066, 0.47583, 0.2855)
    #[test]
    fn the_color_when_a_ray_hits() {
        let w = default_world();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let c = w.color_at(r);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    // Scenario: The color with an intersection behind the ray
    //   Given w ← default_world()
    //     And outer ← the first object in w
    //     And outer.material.ambient ← 1
    //     And inner ← the second object in w
    //     And inner.material.ambient ← 1
    //     And r ← ray(point(0, 0, 0.75), vector(0, 0, -1))
    //   When c ← color_at(w, r)
    //   Then c = inner.material.color
    #[test]
    fn the_color_with_an_intersection_behind_the_ray() {
        let mut w = default_world();
        w.objects[0].material.ambient = 1.0;
        w.objects[1].material.ambient = 1.0;
        let r = ray(point(0.0, 0.0, 0.75), vector(0.0, 0.0, -1.0));
        let c = w.color_at(r);
        assert_eq!(c, w.objects[1].material.color);
    }

    // Scenario: Rendering a world with a camera
    //   Given w ← default_world()
    //     And c ← camera(11, 11, π/2)
    //     And from ← point(0, 0, -5)
    //     And to ← point(0, 0, 0)
    //     And up ← vector(0, 1, 0)
    //     And c.transform ← view_transform(from, to, up)
    //   When image ← render(c, w)
    //   Then pixel_at(image, 5, 5) = color(0.38066, 0.47583, 0.2855)
    #[test]
    fn rendering_a_world_with_a_camera() {
        let w = default_world();
        let mut c = crate::camera::Camera::new(11, 11, PI / 2.0);
        let from = point(0.0, 0.0, -5.0);
        let to = point(0.0, 0.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);
        c.transform = crate::transformations::view_transform(from, to, up);
        let image = render(c, w);
        assert_eq!(image.pixel_at(5, 5), Color::new(0.38066, 0.47583, 0.2855));
    }

    // Scenario: There is no shadow when nothing is collinear with point and light
    //   Given w ← default_world()
    //     And p ← point(0, 10, 0)
    //    Then is_shadowed(w, p) is false
    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = default_world();
        let p = point(0.0, 10.0, 0.0);
        let is_shadowed = w.is_shadowed(p);
        assert!(!is_shadowed);
    }

    // Scenario: The shadow when an object is between the point and the light
    //   Given w ← default_world()
    //     And p ← point(10, -10, 10)
    //    Then is_shadowed(w, p) is true
    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = default_world();
        let p = point(10.0, -10.0, 10.0);
        let is_shadowed = w.is_shadowed(p);
        assert!(is_shadowed);
    }

    // Scenario: There is no shadow when an object is behind the light
    //   Given w ← default_world()
    //     And p ← point(-20, 20, -20)
    //    Then is_shadowed(w, p) is false
    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = default_world();
        let p = point(-20.0, 20.0, -20.0);
        let is_shadowed = w.is_shadowed(p);
        assert!(!is_shadowed);
    }

    // Scenario: There is no shadow when an object is behind the point
    //   Given w ← default_world()
    //     And p ← point(-2, 2, -2)
    //    Then is_shadowed(w, p) is false
    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = default_world();
        let p = point(-2.0, 2.0, -2.0);
        let is_shadowed = w.is_shadowed(p);
        assert!(!is_shadowed);
    }

    // Scenario: shade_hit() is given an intersection in shadow
    //   Given w ← world()
    //     And w.light ← point_light(point(0, 0, -10), color(1, 1, 1))
    //     And s1 ← sphere()
    //     And s1 is added to w
    //     And s2 ← sphere() with:
    //       | transform | translation(0, 0, 10) |
    //     And s2 is added to w
    //     And r ← ray(point(0, 0, 5), vector(0, 0, 1))
    //     And i ← intersection(4, s2)
    //   When comps ← prepare_computations(i, r)
    //     And c ← shade_hit(w, comps)
    //   Then c = color(0.1, 0.1, 0.1)
    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let light = Some(point_light(
            point(0.0, 0.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        ));
        let s1 = Sphere::new();
        let s2 = Sphere::with_transform(crate::transformations::translation(0.0, 0.0, 10.0));
        let w = World {
            light,
            objects: vec![s1, s2],
        };

        let r = ray(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let i = Intersection::new(4.0, &w.objects[1]);
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }

    // Scenario: The hit should offset the point
    //   Given r ← ray(point(0, 0, -5), vector(0, 0, 1))
    //     And shape ← sphere() with:
    //       | transform | translation(0, 0, 1) |
    //     And i ← intersection(5, shape)
    //   When comps ← prepare_computations(i, r)
    //   Then comps.over_point.z < -EPSILON/2
    //     And comps.point.z > comps.over_point.z
    #[test]
    fn the_hit_should_offset_the_point() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut shape = Sphere::new();
        shape.transform = crate::transformations::translation(0.0, 0.0, 1.0);
        let i = Intersection::new(5.0, &shape);
        let comps = i.prepare_computations(r);
        assert!(
            comps.over_point.z < -(EPSILON / 2.0),
            "{:?}",
            comps.over_point
        );
        assert!(comps.point.z > comps.over_point.z);
    }

    // Scenario: The reflected color for a nonreflective material
    //   Given w ← default_world()
    //     And r ← ray(point(0, 0, 0), vector(0, 0, 1))
    //     And shape ← the second object in w
    //     And shape.material.ambient ← 1
    //     And i ← intersection(1, shape)
    //   When comps ← prepare_computations(i, r)
    //     And color ← reflected_color(w, comps)
    //   Then color = color(0, 0, 0)
    #[test]
    fn the_reflected_color_for_a_nonreflective_material() {
        let w = default_world();
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let mut shape = w.objects[1];
        shape.material.ambient = 1.0;
        todo!(
            "let i = Intersection::new(1.0, shape);
        let comps = i.prepare_computations(r);
        let color = w.reflected_color(comps);
        assert_eq!(color, Color::new(0.0, 0.0, 0.0));
        "
        );
    }

    // Scenario: The reflected color for a reflective material
    //   Given w ← default_world()
    //     And shape ← plane() with:
    //       | material.reflective | 0.5                   |
    //       | transform           | translation(0, -1, 0) |
    //     And shape is added to w

    #[test]
    fn the_reflected_color_for_a_reflective_material() {
        let w = default_world();
        let mut shape = Plane::default();
        shape.material.reflective = 0.5;
        shape.transform = crate::transformations::translation(0.0, -1.0, 0.0);
        todo!("w.objects.push(shape);");

        //     And r ← ray(point(0, 0, -3), vector(0, -√2/2, √2/2))
        let r = ray(
            point(0.0, 0.0, -3.0),
            vector(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
        );
        //     And i ← intersection(√2, shape)
        let i = Intersection::new(SQRT_2, &shape);

        //   When comps ← prepare_computations(i, r)
        let comps = i.prepare_computations(r);
        //     And color ← reflected_color(w, comps)
        let color = w.reflected_color(comps);
        //   Then color = color(0.19032, 0.2379, 0.14274)
        assert_eq!(color, Color::new(0.19032, 0.2379, 0.14274));
    }
}
