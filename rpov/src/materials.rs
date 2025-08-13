use crate::floats::Float;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Material {
    pub color: crate::colors::Color,
    pub ambient: Float,
    pub diffuse: Float,
    pub specular: Float,
    pub shininess: Float,
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

    // Scenario: A sphere has a default material
    //   Given s ← sphere()
    //   When m ← s.material
    //   Then m = material()
    #[test]
    fn a_sphere_has_a_default_material() {
        let s = crate::spheres::Sphere::new();
        let m = s.material;
        assert_eq!(m, Material::new());
    }

    // Scenario: A sphere may be assigned a material
    //   Given s ← sphere()
    //     And m ← material()
    //     And m.ambient ← 1
    //   When s.material ← m
    //   Then s.material = m
    #[test]
    fn a_sphere_may_be_assigned_a_material() {
        let mut s = crate::spheres::Sphere::new();
        let mut m = Material::new();
        m.ambient = 1.0;
        s.material = m;
        assert_eq!(s.material, m);
    }
}
