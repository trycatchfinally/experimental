use crate::colors::Color;
use crate::{PointOrVector, Tuple4};

#[derive(Debug, PartialEq)]
pub struct PointLight {
    pub position: Tuple4<f64>,
    pub intensity: Color,
}

pub fn point_light(position: Tuple4<f64>, intensity: Color) -> PointLight {
    assert!(
        position.is_point(),
        "Position must be a point, got {position:?}"
    );
    PointLight {
        position,
        intensity,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::colors::Color;
    use crate::point;

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
}
