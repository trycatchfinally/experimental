// Floating-point precision is configured here
#[cfg(not(feature = "f64"))]
pub type Float = f32;
#[cfg(not(feature = "f64"))]
pub use std::f32::consts;

#[cfg(feature = "f64")]
pub type Float = f64;

#[cfg(feature = "f64")]
pub use std::f64::consts;

pub const ONE: Float = 1.0;
pub const TWO: Float = 2.0;
pub const PI: Float = consts::PI;
pub const SQRT_2: Float = consts::SQRT_2;
// avoids "acne"
pub const EPSILON: Float = 0.0015;

pub const FRAC_1_SQRT_2: Float = consts::FRAC_1_SQRT_2;

pub fn check_float(a: Float, b: Float) {
    assert!((a - b).abs() < EPSILON);
}

#[cfg(test)]
mod tests {
    use super::Float;
    #[test]
    fn check_float_type() {
        if cfg!(feature = "f64") {
            assert_eq!(std::any::type_name::<Float>(), "f64");
        } else {
            assert_eq!(std::any::type_name::<Float>(), "f32");
        }
    }
}
