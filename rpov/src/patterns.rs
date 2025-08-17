// Feature: Patterns
use crate::{colors::Color, intersections::Shape, matrices::Matrix4};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct StripePattern {
    pub a: Color,
    pub b: Color,
    pub transform: Matrix4,
}

pub fn stripe_pattern(a: Color, b: Color) -> StripePattern {
    StripePattern {
        a,
        b,
        transform: Matrix4::identity(),
    }
}
impl StripePattern {
    pub fn stripe_at(&self, point: crate::tuples::Tuple4) -> Color {
        if point.x.floor() as i32 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }

    pub fn stripe_at_object(
        &self,
        object: &dyn Shape,
        world_point: crate::tuples::Tuple4,
    ) -> Color {
        let object_point = object.transform_inverse() * world_point;
        let pattern_point = self.transform.inverse() * object_point;

        self.stripe_at(pattern_point)
    }
}

#[cfg(test)]
mod tests {

    use crate::{colors::Color, patterns::StripePattern, patterns::stripe_pattern, tuples::point};
    fn default_white_black_stripe() -> (Color, Color, StripePattern) {
        let white = Color::new(1.0, 1.0, 1.0);
        let black = Color::new(0.0, 0.0, 0.0);
        (white, black, stripe_pattern(white, black))
    }

    // Scenario: Creating a stripe pattern
    //   Given pattern ← stripe_pattern(white, black)
    //   Then pattern.a = white
    //     And pattern.b = black
    #[test]
    fn creating_a_stripe_pattern() {
        let (white, black, pattern) = default_white_black_stripe();

        assert_eq!(pattern.a, white);
        assert_eq!(pattern.b, black);
    }

    // Scenario: A stripe pattern is constant in y
    //   Given pattern ← stripe_pattern(white, black)
    //   Then stripe_at(pattern, point(0, 0, 0)) = white
    //     And stripe_at(pattern, point(0, 1, 0)) = white
    //     And stripe_at(pattern, point(0, 2, 0)) = white

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let (white, _black, pattern) = default_white_black_stripe();
        assert_eq!(pattern.stripe_at(point(0.0, 0.0, 0.0)), white);
        assert_eq!(pattern.stripe_at(point(0.0, 1.0, 0.0)), white);
        assert_eq!(pattern.stripe_at(point(0.0, 2.0, 0.0)), white);
    }

    // Scenario: A stripe pattern is constant in z
    //   Given pattern ← stripe_pattern(white, black)
    //   Then stripe_at(pattern, point(0, 0, 0)) = white
    //     And stripe_at(pattern, point(0, 0, 1)) = white
    //     And stripe_at(pattern, point(0, 0, 2)) = white
    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let (white, _black, pattern) = default_white_black_stripe();
        assert_eq!(pattern.stripe_at(point(0.0, 0.0, 0.0)), white);
        assert_eq!(pattern.stripe_at(point(0.0, 0.0, 1.0)), white);
        assert_eq!(pattern.stripe_at(point(0.0, 0.0, 2.0)), white);
    }

    // Scenario: A stripe pattern alternates in x
    //   Given pattern ← stripe_pattern(white, black)
    //   Then stripe_at(pattern, point(0, 0, 0)) = white
    //     And stripe_at(pattern, point(0.9, 0, 0)) = white
    //     And stripe_at(pattern, point(1, 0, 0)) = black
    //     And stripe_at(pattern, point(-0.1, 0, 0)) = black
    //     And stripe_at(pattern, point(-1, 0, 0)) = black
    //     And stripe_at(pattern, point(-1.1, 0, 0)) = white
    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let (white, black, pattern) = default_white_black_stripe();
        assert_eq!(pattern.stripe_at(point(0.0, 0.0, 0.0)), white);
        assert_eq!(pattern.stripe_at(point(0.9, 0.0, 0.0)), white);
        assert_eq!(pattern.stripe_at(point(1.0, 0.0, 0.0)), black);
        assert_eq!(pattern.stripe_at(point(-0.1, 0.0, 0.0)), black);
        assert_eq!(pattern.stripe_at(point(-1.0, 0.0, 0.0)), black);
        assert_eq!(pattern.stripe_at(point(-1.1, 0.0, 0.0)), white);
    }

    // Scenario: Lighting with a pattern applied
    //   Given m.pattern ← stripe_pattern(color(1, 1, 1), color(0, 0, 0))
    //     And m.ambient ← 1
    //     And m.diffuse ← 0
    //     And m.specular ← 0
    //     And eyev ← vector(0, 0, -1)
    //     And normalv ← vector(0, 0, -1)
    //     And light ← point_light(point(0, 0, -10), color(1, 1, 1))
    //   When c1 ← lighting(m, light, point(0.9, 0, 0), eyev, normalv, false)
    //     And c2 ← lighting(m, light, point(1.1, 0, 0), eyev, normalv, false)
    //   Then c1 = color(1, 1, 1)
    //     And c2 = color(0, 0, 0)

    #[test]
    fn lighting_with_a_pattern_applied() {
        let mut m = crate::materials::Material::new();
        m.pattern = Some(stripe_pattern(
            Color::new(1.0, 1.0, 1.0),
            Color::new(0.0, 0.0, 0.0),
        ));
        m.ambient = 1.0;
        m.diffuse = 0.0;
        m.specular = 0.0;
        let eyev = crate::tuples::vector(0.0, 0.0, -1.0);
        let normalv = crate::tuples::vector(0.0, 0.0, -1.0);
        let light = crate::lighting::point_light(point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let c1 = crate::lighting::lighting(&m, &light, point(0.9, 0.0, 0.0), eyev, normalv, false);
        let c2 = crate::lighting::lighting(&m, &light, point(1.1, 0.0, 0.0), eyev, normalv, false);
        assert_eq!(c1, Color::new(1.0, 1.0, 1.0));
        assert_eq!(c2, Color::new(0.0, 0.0, 0.0));
    }

    // Scenario: Stripes with an object transformation
    //   Given object ← sphere()
    //     And set_transform(object, scaling(2, 2, 2))
    //     And pattern ← stripe_pattern(white, black)
    //   When c ← stripe_at_object(pattern, object, point(1.5, 0, 0))
    //   Then c = white

    #[test]
    fn stripes_with_an_object_transformation() {
        let mut object = crate::spheres::Sphere::new();
        object.transform = crate::transformations::scaling(2.0, 2.0, 2.0);
        let (white, _black, pattern) = default_white_black_stripe();
        let c = pattern.stripe_at_object(&object, point(1.5, 0.0, 0.0));
        assert_eq!(c, white);
    }

    // Scenario: Stripes with a pattern transformation
    //   Given object ← sphere()
    //     And pattern ← stripe_pattern(white, black)
    //     And set_pattern_transform(pattern, scaling(2, 2, 2))
    //   When c ← stripe_at_object(pattern, object, point(1.5, 0, 0))
    //   Then c = white

    #[test]
    fn stripes_with_a_pattern_transformation() {
        let object = crate::spheres::Sphere::new();
        let (white, _black, mut pattern) = default_white_black_stripe();
        pattern.transform = crate::transformations::scaling(2.0, 2.0, 2.0);
        let c = pattern.stripe_at_object(&object, point(1.5, 0.0, 0.0));
        assert_eq!(c, white);
    }
    // Scenario: Stripes with both an object and a pattern transformation
    //   Given object ← sphere()
    //     And set_transform(object, scaling(2, 2, 2))
    //     And pattern ← stripe_pattern(white, black)
    //     And set_pattern_transform(pattern, translation(0.5, 0, 0))
    //   When c ← stripe_at_object(pattern, object, point(2.5, 0, 0))
    //   Then c = white

    #[test]
    fn stripes_with_both_an_object_and_a_pattern_transformation() {
        let mut object = crate::spheres::Sphere::new();
        object.transform = crate::transformations::scaling(2.0, 2.0, 2.0);
        let (white, _black, mut pattern) = default_white_black_stripe();
        pattern.transform = crate::transformations::translation(0.5, 0.0, 0.0);
        let c = pattern.stripe_at_object(&object, point(2.5, 0.0, 0.0));
        assert_eq!(c, white);
    }

    // Stop here.

    // Scenario: The default pattern transformation
    //   Given pattern ← test_pattern()
    //   Then pattern.transform = identity_matrix

    // Scenario: Assigning a transformation to a pattern
    //   Given pattern ← test_pattern()
    //   When set_pattern_transform(pattern, translation(1, 2, 3))
    //   Then pattern.transform = translation(1, 2, 3)

    // Scenario: A pattern with an object transformation
    //   Given shape ← sphere()
    //     And set_transform(shape, scaling(2, 2, 2))
    //     And pattern ← test_pattern()
    //   When c ← pattern_at_shape(pattern, shape, point(2, 3, 4))
    //   Then c = color(1, 1.5, 2)

    // Scenario: A pattern with a pattern transformation
    //   Given shape ← sphere()
    //     And pattern ← test_pattern()
    //     And set_pattern_transform(pattern, scaling(2, 2, 2))
    //   When c ← pattern_at_shape(pattern, shape, point(2, 3, 4))
    //   Then c = color(1, 1.5, 2)

    // Scenario: A pattern with both an object and a pattern transformation
    //   Given shape ← sphere()
    //     And set_transform(shape, scaling(2, 2, 2))
    //     And pattern ← test_pattern()
    //     And set_pattern_transform(pattern, translation(0.5, 1, 1.5))
    //   When c ← pattern_at_shape(pattern, shape, point(2.5, 3, 3.5))
    //   Then c = color(0.75, 0.5, 0.25)

    // Scenario: A gradient linearly interpolates between colors
    //   Given pattern ← gradient_pattern(white, black)
    //   Then pattern_at(pattern, point(0, 0, 0)) = white
    //     And pattern_at(pattern, point(0.25, 0, 0)) = color(0.75, 0.75, 0.75)
    //     And pattern_at(pattern, point(0.5, 0, 0)) = color(0.5, 0.5, 0.5)
    //     And pattern_at(pattern, point(0.75, 0, 0)) = color(0.25, 0.25, 0.25)

    // Scenario: A ring should extend in both x and z
    //   Given pattern ← ring_pattern(white, black)
    //   Then pattern_at(pattern, point(0, 0, 0)) = white
    //     And pattern_at(pattern, point(1, 0, 0)) = black
    //     And pattern_at(pattern, point(0, 0, 1)) = black
    //     # 0.708 = just slightly more than √2/2
    //     And pattern_at(pattern, point(0.708, 0, 0.708)) = black

    // Scenario: Checkers should repeat in x
    //   Given pattern ← checkers_pattern(white, black)
    //   Then pattern_at(pattern, point(0, 0, 0)) = white
    //     And pattern_at(pattern, point(0.99, 0, 0)) = white
    //     And pattern_at(pattern, point(1.01, 0, 0)) = black

    // Scenario: Checkers should repeat in y
    //   Given pattern ← checkers_pattern(white, black)
    //   Then pattern_at(pattern, point(0, 0, 0)) = white
    //     And pattern_at(pattern, point(0, 0.99, 0)) = white
    //     And pattern_at(pattern, point(0, 1.01, 0)) = black

    // Scenario: Checkers should repeat in z
    //   Given pattern ← checkers_pattern(white, black)
    //   Then pattern_at(pattern, point(0, 0, 0)) = white
    //     And pattern_at(pattern, point(0, 0, 0.99)) = white
    //     And pattern_at(pattern, point(0, 0, 1.01)) = black
}
