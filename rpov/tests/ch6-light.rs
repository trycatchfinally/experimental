mod test {

    use indicatif::{ProgressBar, ProgressStyle};
    use num_traits::ToPrimitive;
    use rpov::{
        canvas::Canvas,
        intersections::hit,
        lighting::{lighting, point_light},
        spheres::Sphere,
        tuples::Tuple4,
        tuples::point,
    };

    fn run_example(name: &str, canvas_pixels: usize) {
        let ray_origin = point(0.0, 0.0, -5.0);
        let wall_z = 10.0;
        let wall_size = 7.0;
        let pixel_size = wall_size / (canvas_pixels.to_f32().unwrap());
        let half = wall_size / 2.0;
        let mut c = Canvas::new(canvas_pixels, canvas_pixels);
        let mut shape = Sphere::new();
        shape.material.color = rpov::colors::Color::new(1.0, 0.2, 1.0);

        let light_position = point(-10.0, 10.0, -10.0);
        let light_color = rpov::colors::Color::new(1.0, 1.0, 1.0);
        let light = point_light(light_position, light_color);

        let path = format!("tests/out-ch6-{name}-{canvas_pixels}x{canvas_pixels}.ppm");
        let mut found = 0;
        let bar = ProgressBar::new(canvas_pixels as u64);
        bar.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta}) {msg}")
            .unwrap()
            .progress_chars("#>- "));
        bar.set_message(format!("Rendering {path}"));

        for y in 0..canvas_pixels {
            bar.inc(1);
            let world_y = half - pixel_size * y.to_f32().unwrap();
            for x in 0..canvas_pixels {
                let world_x = -half + pixel_size * x.to_f32().unwrap();
                let position: Tuple4 = point(world_x.into(), world_y.into(), wall_z);
                let r = rpov::rays::ray(ray_origin, (position - ray_origin).normalize());
                let intersections = shape.intersect(r);

                let i = hit(&intersections);
                if i.is_none() {
                    continue;
                }
                found += 1;
                let hit = i.unwrap();
                let point = r.position(hit.t);
                let normal = hit.object.normal_at(&point);
                let eye = -r.direction;

                let color = lighting(&hit.object.material, &light, point, eye, normal);
                c.write_pixel(x, y, color);
            }
        }
        bar.finish_with_message(format!("Rendering {path} complete!"));

        std::fs::write(&path, c.to_ppm()).expect("Unable to write file");
        assert!(found > 0, "No intersections found");
    }

    #[test]
    fn ch6_putting_it_together() {
        run_example("identity", 50);
    }

    #[test]
    #[cfg(not(debug_assertions))]
    fn release_generation() {
        run_example("release", 1600);
    }
}
