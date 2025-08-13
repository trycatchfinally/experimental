use rpov::camera::Camera;
use rpov::colors::Color;
use rpov::floats::consts::PI;
use rpov::lighting::point_light;
use rpov::materials::Material;
use rpov::spheres::Sphere;
use rpov::transformations::{rotation_x, rotation_y, scaling, translation, view_transform};
use rpov::tuples::{point, vector};
use rpov::world::{World, render};

mod tests {
    use super::*;

    fn render_scenario_7(hs: usize, vs: usize) {
        let mut floor = Sphere::new();
        floor.transform = scaling(10.0, 0.01, 10.0);
        let mut floor_material = Material::new();
        floor_material.color = Color::new(1.0, 0.9, 0.9);
        floor_material.specular = 0.0;
        floor.material = floor_material;

        let mut left_wall = Sphere::new();
        left_wall.transform = translation(0.0, 0.0, 5.0)
            * rotation_y(-PI / 4.0)
            * rotation_x(PI / 2.0)
            * scaling(10.0, 0.01, 10.0);
        left_wall.material = floor.material;

        let mut right_wall = Sphere::new();
        right_wall.transform = translation(0.0, 0.0, 5.0)
            * rotation_y(PI / 4.0)
            * rotation_x(PI / 2.0)
            * scaling(10.0, 0.01, 10.0);
        right_wall.material = floor.material;

        let mut middle = Sphere::new();
        middle.transform = translation(-0.5, 1.0, 0.5);
        let mut middle_material = Material::new();
        middle_material.color = Color::new(0.1, 1.0, 0.5);
        middle_material.diffuse = 0.7;
        middle_material.specular = 0.3;
        middle.material = middle_material;

        let mut right = Sphere::new();
        right.transform = translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5);
        let mut right_material = Material::new();
        right_material.color = Color::new(0.5, 1.0, 0.1);
        right_material.diffuse = 0.7;
        right_material.specular = 0.3;
        right.material = right_material;

        let mut left = Sphere::new();
        left.transform = translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33);
        let mut left_material = Material::new();
        left_material.color = Color::new(1.0, 0.8, 0.1);
        left_material.diffuse = 0.7;
        left_material.specular = 0.3;
        left.material = left_material;

        let mut world = World::new();
        world.objects = vec![floor, left_wall, right_wall, middle, right, left];
        world.light = Some(point_light(
            point(-10.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        ));

        let mut camera = Camera::new(hs, vs, PI / 3.0);
        camera.transform = view_transform(
            point(0.0, 1.5, -5.0),
            point(0.0, 1.0, 0.0),
            vector(0.0, 1.0, 0.0),
        );

        let canvas = render(camera, world);
        let ppm = canvas.to_ppm();
        let path = format!("tests/out-ch7-scene-{hs}x{vs}.ppm");
        std::fs::write(path, ppm).unwrap();
    }
    #[test]
    fn scenario_7() {
        render_scenario_7(200, 100);
    }

    #[test]
    #[cfg(not(debug_assertions))]
    fn release_generation() {
        render_scenario_7(3200, 1600);
    }
}
