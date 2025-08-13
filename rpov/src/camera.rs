use crate::{
    floats::Float,
    matrices::Matrix4,
    rays::{Ray, ray},
    tuples::point,
};

pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub field_of_view: Float,
    pub transform: Matrix4,
    pub pixel_size: Float,
    half_width: Float,
    half_height: Float,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: Float) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect_ratio = hsize as Float / vsize as Float;

        let (half_width, half_height) = if aspect_ratio >= 1.0 {
            (half_view, half_view / aspect_ratio)
        } else {
            (half_view * aspect_ratio, half_view)
        };
        let pixel_size = (half_width * 2.0) / hsize as Float;

        Camera {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix4::identity(),
            pixel_size,
            half_width,
            half_height,
        }
    }

    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        let xoffset = (px as Float + 0.5) * self.pixel_size;
        let yoffset = (py as Float + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let pixel = self.transform.inverse() * point(world_x, world_y, -1.0);
        let origin = self.transform.inverse() * point(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();

        ray(origin, direction)
    }
}

#[cfg(test)]
mod tests {
    use crate::floats::check_float;
    use crate::floats::consts::FRAC_1_SQRT_2;
    use crate::floats::consts::PI;
    use crate::transformations::{rotation_y, translation};
    use crate::tuples::check_tuple;
    use crate::{
        camera::Camera,
        matrices::Matrix4,
        tuples::{point, vector},
    };

    // Scenario: Constructing a camera
    //   Given hsize ← 160
    //     And vsize ← 120
    //     And field_of_view ← π/2
    //   When c ← camera(hsize, vsize, field_of_view)
    //   Then c.hsize = 160
    //     And c.vsize = 120
    //     And c.field_of_view = π/2
    //     And c.transform = identity_matrix
    #[test]
    fn constructing_a_camera() {
        let c = Camera::new(160, 120, PI / 2.0);
        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert_eq!(c.field_of_view, PI / 2.0);
        assert_eq!(c.transform, Matrix4::identity());
    }

    // Scenario: The pixel size for a horizontal canvas
    //   Given c ← camera(200, 125, π/2)
    //   Then c.pixel_size = 0.01
    #[test]
    fn the_pixel_size_for_a_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);
        check_float(c.pixel_size, 0.01);
    }

    // Scenario: The pixel size for a vertical canvas
    //   Given c ← camera(125, 200, π/2)
    //   Then c.pixel_size = 0.01
    #[test]
    fn the_pixel_size_for_a_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);
        check_float(c.pixel_size, 0.01);
    }

    // Scenario: Constructing a ray through the center of the canvas
    //   Given c ← camera(201, 101, π/2)
    //   When r ← ray_for_pixel(c, 100, 50)
    //   Then r.origin = point(0, 0, 0)
    //     And r.direction = vector(0, 0, -1)
    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50);
        check_tuple(r.origin, point(0.0, 0.0, 0.0));
        check_tuple(r.direction, vector(0.0, 0.0, -1.0));
    }

    // Scenario: Constructing a ray through a corner of the canvas
    //   Given c ← camera(201, 101, π/2)
    //   When r ← ray_for_pixel(c, 0, 0)
    //   Then r.origin = point(0, 0, 0)
    //     And r.direction = vector(0.66519, 0.33259, -0.66851)
    #[test]
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0, 0);
        check_tuple(r.origin, point(0.0, 0.0, 0.0));
        check_tuple(r.direction, vector(0.66519, 0.33259, -0.66851));
    }

    // Scenario: Constructing a ray when the camera is transformed
    //   Given c ← camera(201, 101, π/2)
    //   When c.transform ← rotation_y(π/4) * translation(0, -2, 5)
    //     And r ← ray_for_pixel(c, 100, 50)
    //   Then r.origin = point(0, 2, -5)
    //     And r.direction = vector(√2/2, 0, -√2/2)

    #[test]
    fn constructing_a_ray_when_the_camera_is_transformed() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.transform = rotation_y(PI / 4.0) * translation(0.0, -2.0, 5.0);
        let r = c.ray_for_pixel(100, 50);
        check_tuple(r.origin, point(0.0, 2.0, -5.0));
        check_tuple(r.direction, vector(FRAC_1_SQRT_2, 0.0, -FRAC_1_SQRT_2));
    }
}
