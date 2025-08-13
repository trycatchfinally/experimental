use crate::colors::{COLOR_BLACK, Color};
use crate::materials::Material;
use crate::tuples::{PointOrVector, Tuple4};

#[derive(Debug, PartialEq)]
pub struct PointLight {
    pub position: Tuple4,
    pub intensity: Color,
}

pub fn point_light(position: Tuple4, intensity: Color) -> PointLight {
    assert!(
        position.is_point(),
        "Position must be a point, got {position:?}"
    );
    PointLight {
        position,
        intensity,
    }
}

pub fn lighting(
    material: &Material,
    light: &PointLight,
    position: Tuple4,
    eyev: Tuple4,
    normalv: Tuple4,
) -> Color {
    // combine the surface color with the light's color/intensity
    let effective_color = material.color * light.intensity;

    // find the direction to the light source
    let lightv = (light.position - position).normalize();

    // compute the ambient contribution
    let ambient = effective_color * material.ambient;

    // light_dot_normal represents the cosine of the angle between the
    // light vector and the normal vector. A negative number means the
    // light is on the other side of the.surface.
    let light_dot_normal = lightv.dot(normalv);

    let (diffuse, specular) = if light_dot_normal < 0.0 {
        (COLOR_BLACK, COLOR_BLACK)
    } else {
        // compute the diffuse contribution
        let diffuse = effective_color * material.diffuse * light_dot_normal;

        // reflect_dot_eye represents the cosine of the angle between the
        // reflection vector and the eye vector. A negative number means the
        // light reflects away from the eye.
        let reflectv = (-lightv).reflect(normalv);
        let reflect_dot_eye = reflectv.dot(eyev);

        if reflect_dot_eye <= 0.0 {
            (diffuse, COLOR_BLACK)
        } else {
            // compute the specular contribution
            let factor = reflect_dot_eye.powf(material.shininess);
            let specular = light.intensity * material.specular * factor;
            (diffuse, specular)
        }
    };

    ambient + diffuse + specular
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::colors::Color;
    use crate::materials::Material;
    use crate::tuples::{point, vector};

    // Scenario: A point light has a position and intensity
    //   Given intensity ← color(1, 1, 1)
    //     And position ← point(0, 0, 0)
    //   When light ← point_light(position, intensity)
    //   Then light.position = position
    //     And light.intensity = intensity
    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = point(0.0, 0.0, 0.0);
        let light = point_light(position, intensity);
        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }

    fn setup() -> (Material, Tuple4) {
        (Material::new(), point(0.0, 0.0, 0.0))
    }

    // Scenario: Lighting with the eye between the light and the surface
    #[test]
    fn test_lighting_with_eye_between_light_and_surface() {
        let (m, position) = setup();
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(&m, &light, position, eyev, normalv);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    // Scenario: Lighting with the eye between light and surface, eye offset 45°
    #[test]
    fn test_lighting_with_eye_offset_45_degrees() {
        let (m, position) = setup();
        let two = crate::floats::TWO;
        let eyev = vector(0.0, two.sqrt() / 2.0, -(two.sqrt() / 2.0));
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(&m, &light, position, eyev, normalv);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    // Scenario: Lighting with eye opposite surface, light offset 45°
    #[test]
    fn test_lighting_with_eye_opposite_surface_light_offset_45_degrees() {
        let (m, position) = setup();
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(&m, &light, position, eyev, normalv);
        assert_eq!(
            result,
            Color::new(0.7363961030678928, 0.7363961030678928, 0.7363961030678928)
        );
    }

    // Scenario: Lighting with eye in the path of the reflection vector
    #[test]
    fn test_lighting_with_eye_in_path_of_reflection_vector() {
        let (m, position) = setup();
        let two = crate::floats::TWO;
        let eyev = vector(0.0, -two.sqrt() / 2.0, -two.sqrt() / 2.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(&m, &light, position, eyev, normalv);
        assert_eq!(
            result,
            Color::new(1.6363961030678928, 1.6363961030678928, 1.6363961030678928)
        );
    }

    // Scenario: Lighting with the light behind the surface
    #[test]
    fn test_lighting_with_light_behind_surface() {
        let (m, position) = setup();
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = point_light(point(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(&m, &light, position, eyev, normalv);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}
