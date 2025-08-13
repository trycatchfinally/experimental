use crate::{colors::Color, floats::Float};

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            pixels: vec![Color::new(0.0, 0.0, 0.0); width * height],
        }
    }

    fn check_xy(&self, x: usize, y: usize) -> usize {
        assert!(
            x < self.width && y < self.height,
            "Pixel coordinates: x={}, y={} are out of bounds: width={} height={}",
            x,
            y,
            self.width,
            self.height
        );
        y * self.width + x
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        let pos = self.check_xy(x, y);
        self.pixels[pos] = color;
    }

    pub fn write_block(
        &mut self,
        x: usize,
        y: usize,
        x_direction: i32,
        y_direction: i32,
        color: Color,
    ) {
        for i in 0..x_direction {
            for j in 0..y_direction {
                self.write_pixel(x + i as usize, y + j as usize, color);
            }
        }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        let pos = self.check_xy(x, y);
        self.pixels[pos]
    }

    pub fn to_ppm(&self) -> String {
        let mut ppm = String::new();
        ppm.push_str("P3\n");
        ppm.push_str(&format!("{} {}\n", self.width, self.height));
        ppm.push_str("255\n");

        for y in 0..self.height {
            let mut line = String::new();
            let mut line_len = 0;
            for x in 0..self.width {
                let color = self.pixel_at(x, y);
                let (r, g, b) = (
                    Canvas::scale_color(color.red),
                    Canvas::scale_color(color.green),
                    Canvas::scale_color(color.blue),
                );
                for val in [r, g, b] {
                    let s = val.to_string();
                    // +1 for the space if not first in line
                    let extra = if line_len == 0 { 0 } else { 1 };
                    if line_len + s.len() + extra > 70 {
                        ppm.push_str(line.trim_end());
                        ppm.push('\n');
                        line.clear();
                        line_len = 0;
                    }
                    if line_len > 0 {
                        line.push(' ');
                        line_len += 1;
                    }
                    line.push_str(&s);
                    line_len += s.len();
                    assert!(line_len <= 70, "Line length exceeded 70 characters");
                }
            }
            ppm.push_str(line.trim_end());
            ppm.push('\n');
        }
        if !ppm.ends_with('\n') {
            ppm.push('\n');
        }
        ppm
    }

    fn scale_color(c: Float) -> u8 {
        let c = c.clamp(0.0, 1.0);
        (c * 255.0).round() as u8
    }
}

// ...existing

#[cfg(test)]
mod tests {
    use super::*;
    use crate::colors::{COLOR_BLACK, Color};

    /*
    Scenario: Creating a canvas
      Given c ← canvas(10, 20)
      Then c.width = 10
        And c.height = 20
        And every pixel of c is color(0, 0, 0)
    */
    #[test]
    fn creating_a_canvas() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        let zero = Color::new(0.0, 0.0, 0.0);
        for y in 0..c.height {
            for x in 0..c.width {
                assert_eq!(c.pixel_at(x, y), zero);
            }
        }
    }

    /*
    Scenario: Writing pixels to a canvas
      Given c ← canvas(10, 20)
        And red ← color(1, 0, 0)
      When write_pixel(c, 2, 3, red)
      Then pixel_at(c, 2, 3) = red
    */
    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        c.write_pixel(2, 3, red);
        assert_eq!(c.pixel_at(2, 3), red);
    }

    /*
    Scenario: Constructing the PPM header
      Given c ← canvas(5, 3)
      When ppm ← canvas_to_ppm(c)
      Then lines 1-3 of ppm are
        """
        P3
        5 3
        255
        """
    */
    #[test]
    fn constructing_the_ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();
        let lines: Vec<&str> = ppm.lines().collect();
        assert_eq!(lines[0], "P3");
        assert_eq!(lines[1], "5 3");
        assert_eq!(lines[2], "255");
    }

    /*
    Scenario: Constructing the PPM pixel data
      Given c ← canvas(5, 3)
        And c1 ← color(1.5, 0, 0)
        And c2 ← color(0, 0.5, 0)
        And c3 ← color(-0.5, 0, 1)
      When write_pixel(c, 0, 0, c1)
        And write_pixel(c, 2, 1, c2)
        And write_pixel(c, 4, 2, c3)
        And ppm ← canvas_to_ppm(c)
      Then lines 4-6 of ppm are
        """
        255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0 0 0 0 255
        """
    */
    #[test]
    fn constructing_the_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);
        let ppm = c.to_ppm();
        let lines: Vec<&str> = ppm.lines().collect();
        assert_eq!(lines[3], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
        assert_eq!(lines[4], "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
        assert_eq!(lines[5], "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
    }

    /*
    Scenario: Splitting long lines in PPM files
      Given c ← canvas(10, 2)
      When every pixel of c is set to color(1, 0.8, 0.6)
        And ppm ← canvas_to_ppm(c)
      Then lines 4-7 of ppm are
        """
        255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
        153 255 204 153 255 204 153 255 204 153 255 204 153
        255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
        153 255 204 153 255 204 153 255 204 153 255 204 153
        """
    */
    #[test]
    fn splitting_long_lines_in_ppm_files() {
        let mut c = Canvas::new(10, 2);
        let color = Color::new(1.0, 0.8, 0.6);
        for y in 0..2 {
            for x in 0..10 {
                c.write_pixel(x, y, color);
            }
        }
        let ppm = c.to_ppm();
        let lines: Vec<&str> = ppm.lines().collect();
        assert_eq!(
            lines[3],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
        );
        assert_eq!(
            lines[4],
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
        assert_eq!(
            lines[5],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
        );
        assert_eq!(
            lines[6],
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
    }

    /*
    Scenario: PPM files are terminated by a newline character
      Given c ← canvas(5, 3)
      When ppm ← canvas_to_ppm(c)
      Then ppm ends with a newline character
    */
    #[test]
    fn ppm_files_are_terminated_by_a_newline_character() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();
        assert!(ppm.ends_with('\n'));
    }

    #[test]
    fn test_write_block_positive() {
        let mut c = Canvas::new(5, 5);
        let black = COLOR_BLACK;
        let color = Color::new(0.5, 0.5, 0.5);
        c.write_block(1, 1, 3, 2, color);
        assert_eq!(c.pixel_at(0, 0), black);
        assert_eq!(c.pixel_at(1, 0), black);
        assert_eq!(c.pixel_at(1, 1), color);
        assert_eq!(c.pixel_at(2, 1), color);
        assert_eq!(c.pixel_at(3, 1), color);
        assert_eq!(c.pixel_at(1, 2), color);
        assert_eq!(c.pixel_at(2, 2), color);
        assert_eq!(c.pixel_at(3, 2), color);
        assert_eq!(c.pixel_at(4, 4), black);
    }
}
