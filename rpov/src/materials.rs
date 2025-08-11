#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Material {
    pub color: crate::colors::Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Default for Material {
    fn default() -> Self {
        Self::new()
    }
}

impl Material {
    pub fn new() -> Self {
        Material {
            color: crate::colors::Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}
// Scenario: The default material
//   Given m ← material()
//   Then m.color = color(1, 1, 1)
//     And m.ambient = 0.1
//     And m.diffuse = 0.9
//     And m.specular = 0.9
//     And m.shininess = 200.0
#[cfg(test)]
mod tests {
    use super::*;
    use crate::colors::Color;

    #[test]
    fn the_default_material() {
        let m = Material::new();
        assert_eq!(m.color, Color::new(1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }
}
