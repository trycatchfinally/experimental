struct Projectile {
    position: rpov::Tuple,
    velocity: rpov::Tuple,
}

struct Environment {
    gravity: rpov::Tuple,
    wind: rpov::Tuple,
}

fn tick_projectile(env: &Environment, proj: &Projectile) -> Projectile {
    use rpov::PlusMinus;
    let new_position = proj.position.plus(proj.velocity);
    let new_velocity = proj.velocity.plus(env.gravity).plus(env.wind);
    Projectile {
        position: new_position,
        velocity: new_velocity,
    }
}

mod test {
    use num_traits::AsPrimitive;
    use rpov::colors::COLOR_WHITE;
    use rpov::{make_point, make_vector};

    use crate::Environment;
    use crate::{Projectile, tick_projectile};
    use rpov::canvas::Canvas;

    #[test]
    pub fn test_simulate() {
        let start = make_point(0.0, 1.0, 0.0);
        let velocity = make_vector(1.0, 1.8, 0.0).normalize() * 11.25;
        let mut projectile = Projectile {
            position: start,
            velocity: velocity,
        };
        let gravity = make_vector(0.0, -0.1, 0.0);
        let wind = make_vector(-0.01, 0.0, 0.0);
        let e = Environment { gravity, wind };
        let mut c = Canvas::new(900, 550);

        let mut tick = 0;
        let red = rpov::colors::Color::new(1.0, 0.0, 0.0);
        let mut max_speed: f64 = projectile.velocity.magnitude().as_();
        while projectile.position.y > 0.0 {
            projectile = tick_projectile(&e, &projectile);
            let x = projectile.position.x.round() as usize;
            let y = projectile.position.y.round() as usize;
            if y >= c.height || y <= 0 {
                continue;
            }
            // if x >= c.width || y >= c.height {
            //     panic!("Projectile out of bounds at tick {}: x={}, y={}", tick, x, y);
            // }
            let inv_y = c.height - y - 1; // Invert y for canvas coordinates
            let speed: f64 = projectile.velocity.magnitude().as_();
            let scaled_red = red * (speed / max_speed);
            max_speed = max_speed.max(speed);
            c.write_block(x, inv_y, 3, 3, scaled_red);
            c.write_pixel(x, inv_y, red);
            tick += 1;
        }
        c.write_pixel(c.width / 2, c.height / 2, COLOR_WHITE);
        let path = format!("tests/out-projectile-{}.ppm", tick);
        std::fs::write(&path, c.to_ppm()).expect("Unable to write file");
    }
}
