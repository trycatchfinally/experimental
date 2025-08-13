mod test {
    use rpov::floats::consts::PI;

    use num_traits::ToPrimitive;
    use rpov::{
        canvas::Canvas,
        colors::COLOR_RED,
        matrices::Matrix4,
        spheres::Sphere,
        transformations::{rotation_z, scaling, shearing},
        tuples::Tuple4,
        tuples::point,
    };

    fn run_example(name: &str, transform: Matrix4) {
        let ray_origin = point(0.0, 0.0, -5.0);
        let wall_z = -10.0;
        let wall_size = 7.0;
        let canvas_pixels = 100;
        let pixel_size = wall_size / (canvas_pixels.to_f32().unwrap());
        let half = wall_size / 2.0;
        let mut c = Canvas::new(canvas_pixels, canvas_pixels);
        let color = COLOR_RED;
        let shape = Sphere::with_transform(transform);

        for y in 0..canvas_pixels {
            let world_y = half - pixel_size * y.to_f32().unwrap();
            for x in 0..canvas_pixels {
                let world_x = -half + pixel_size * x.to_f32().unwrap();
                let position: Tuple4 = point(world_x.into(), world_y.into(), wall_z);
                let r = rpov::rays::ray(ray_origin, (position - ray_origin).normalize());
                let xs = shape.intersect(r);

                if !xs.is_empty() {
                    c.write_pixel(x, y, color);
                }
            }
        }
        let path = format!("tests/out-ch5-{name}.ppm");
        std::fs::write(&path, c.to_ppm()).expect("Unable to write file");
    }

    #[test]
    fn ch5_putting_it_together() {
        let sx = scaling(0.5, 1.0, 1.0);
        run_example("identity", Matrix4::identity());
        run_example("scaling-y", scaling(1.0, 0.5, 1.0));
        run_example("scaling-x", sx);

        let shr = rotation_z(PI / 4.0) * scaling(0.5, 1.0, 1.0);
        run_example("shrink-rotation", shr);

        let skr = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0) * sx;
        run_example("shearing", skr);
    }
}
